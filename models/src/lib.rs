use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait CsvWrite {
    fn write_form(&self) -> String;
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct NewParticleCount {
    pub micro_meter_10: u64,
    pub micro_meter_60: u64,
    pub micro_meter_180: u64,
    pub micro_meter_500: u64,
}

impl NewParticleCount {
    pub fn new(
        micro_meter_10: u64,
        micro_meter_60: u64,
        micro_meter_180: u64,
        micro_meter_500: u64,
    ) -> Self {
        Self {
            micro_meter_10,
            micro_meter_60,
            micro_meter_180,
            micro_meter_500,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParticleCount {
    id: String,
    micro_meter_10: u64,
    micro_meter_60: u64,
    micro_meter_180: u64,
    micro_meter_500: u64,
    time: DateTime<Utc>,
}

impl ParticleCount {
    pub fn new(
        id: String,
        mm10: u64,
        mm60: u64,
        mm180: u64,
        mm500: u64,
        time: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            micro_meter_10: mm10,
            micro_meter_60: mm60,
            micro_meter_180: mm180,
            micro_meter_500: mm500,
            time,
        }
    }
}

impl From<NewParticleCount> for ParticleCount {
    fn from(value: NewParticleCount) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            micro_meter_10: value.micro_meter_10,
            micro_meter_60: value.micro_meter_60,
            micro_meter_180: value.micro_meter_180,
            micro_meter_500: value.micro_meter_500,
            time: Utc::now(),
        }
    }
}

impl CsvWrite for ParticleCount {
    fn write_form(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            &self.id,
            &self.micro_meter_10,
            &self.micro_meter_60,
            &self.micro_meter_180,
            &self.micro_meter_500,
            &self.time
        )
    }
}

#[derive(Debug, Clone)]
pub enum DisplayError {
    Serde(String),
    NumParseError(String),
    FileReadError(String),
    U8parseError(String),
    ConvertToU64Error(String),
    WriteError(String),
}
