use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use csv::ReaderBuilder;
use models::{CsvWrite, DisplayError, ParticleCount};

pub fn read_file(path: impl AsRef<Path>) -> Result<String, DisplayError> {
    fs::read(path)
        .map_err(|e| DisplayError::FileReadError(e.to_string()))
        .and_then(|result| {
            String::from_utf8(result).map_err(|e| DisplayError::U8parseError(e.to_string()))
        })
}

pub fn does_file_exist(path: impl AsRef<Path>) -> bool {
    // Canonicalize fails when the path does not point to anything. Use this fact to map to bool
    fs::canonicalize(path).map(|_| true).unwrap_or(false)
}

pub fn parse_data(path: impl AsRef<Path>) -> Result<Vec<ParticleCount>, DisplayError> {
    File::open(path)
        .map_err(|e| DisplayError::FileReadError(e.to_string()))
        .and_then(|file| {
            ReaderBuilder::new()
                .flexible(true)
                .from_reader(file)
                .deserialize()
                .map(|record| record.map_err(|e| DisplayError::Serde(e.to_string())))
                .collect::<Result<Vec<ParticleCount>, DisplayError>>()
        })
}

pub fn write_data(path: impl AsRef<Path>, data: &ParticleCount) -> Result<(), DisplayError> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|e| DisplayError::FileReadError(e.to_string()))?;

    writeln!(&mut file, "{}", data.write_form())
        .map_err(|e| DisplayError::WriteError(e.to_string()))
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod read_file_should {
        use crate::read_file;

        #[test]
        fn return_file_content_as_string() {
            let string = read_file("data/test.csv").unwrap();

            let first_line = string.lines().next().unwrap().trim();
            assert_eq!(
                first_line,
                "id,micro_meter_10,micro_meter_60,micro_meter_180,micro_meter_500,time"
            );
        }

        #[test]
        fn error_when_file_doesnt_exist() {
            let result = read_file("data/some_nonsense.csv");

            assert!(result.is_err(), "Result should have been an error");

            // I don't care what the inner string is. Only that it was mapped to DisplayError correctly.
            let error = result.unwrap_err();
            match error {
                models::DisplayError::Serde(_) => unreachable!(),
                models::DisplayError::NumParseError(_) => unreachable!(),
                models::DisplayError::FileReadError(_) => (),
                models::DisplayError::U8parseError(_) => unreachable!(),
                models::DisplayError::WriteError(_) => unreachable!(),
                models::DisplayError::ConvertToU64Error(_) => unreachable!(),
            }
        }
    }

    #[cfg(test)]
    mod does_file_exist_should {
        use crate::does_file_exist;

        #[test]
        fn return_true_when_path_exists() {
            let exists = does_file_exist("data/test.csv");

            assert!(exists, "File did not exist and should have");
        }

        #[test]
        fn return_false_when_path_is_nonsense() {
            let exists = does_file_exist("data/some_nonsense.csv");

            assert!(!exists, "File did exist and shouldn't");
        }
    }

    #[cfg(test)]
    mod parse_data_should {
        use chrono::{DateTime, Utc};
        use models::ParticleCount;

        use crate::parse_data;

        #[test]
        fn parse_data_in_test_file() {
            println!("{}", Utc::now());
            let path = "data/test.csv";

            let particle_counts = parse_data(path).unwrap();
            assert_eq!(
                particle_counts,
                vec![ParticleCount::new(
                    "ba31bb3c-cf22-4635-88e6-1e12b5cceae0".to_string(),
                    50000,
                    25000,
                    10000,
                    4000,
                    "2023-08-19T22:32:49.582287501Z"
                        .parse::<DateTime<Utc>>()
                        .unwrap()
                )]
            );
        }
    }

    #[cfg(test)]
    mod write_data_should {
        use std::{
            fs::{File, OpenOptions},
            io,
            sync::atomic::AtomicU8,
        };

        use chrono::Utc;
        use models::ParticleCount;
        use rand::Rng;
        use uuid::Uuid;

        use crate::{parse_data, write_data};

        const DATA_PATH: &str = "data/test.csv";
        static ID: AtomicU8 = AtomicU8::new(0);

        fn copy_data_file(copy_file: &str) -> String {
            let new_path = format!(
                "data/test-{}.csv",
                ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            );

            let mut copy_file = File::open(copy_file).unwrap();
            let mut file = OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(&new_path)
                .unwrap();
            let written = io::copy(&mut copy_file, &mut file).unwrap();

            assert_ne!(written, 0);
            new_path
        }

        #[test]
        #[ignore = "Creates and deletes a file, not meant to be run every change"]
        fn append_file() {
            let new_path = copy_data_file(DATA_PATH);
            let mut random_generator = rand::thread_rng();

            let randoms = (0..4)
                .map(|_| random_generator.gen_range(10000..275000))
                .collect::<Vec<u64>>();

            let particle_count = ParticleCount::new(
                Uuid::new_v4().to_string(),
                randoms[0],
                randoms[1],
                randoms[2],
                randoms[3],
                Utc::now(),
            );

            write_data(&new_path, &particle_count).unwrap();

            let particle_counts = parse_data(&new_path).unwrap();
            let found = particle_counts
                .into_iter()
                .find(|count| count == &particle_count);

            assert!(
                found.is_some(),
                "Unable to find the new data in file {}",
                &new_path
            );

            std::fs::remove_file(&new_path).unwrap();
        }
    }
}
