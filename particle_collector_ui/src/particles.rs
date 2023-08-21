use chrono::Utc;
use models::{DisplayError, ParticleCount};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct NewParticle {
    pub micro_meter_10: String,
    pub micro_meter_60: String,
    pub micro_meter_180: String,
    pub micro_meter_500: String,
}

impl Into<Result<ParticleCount, DisplayError>> for NewParticle {
    fn into(self) -> Result<ParticleCount, DisplayError> {
        vec![
            self.micro_meter_10
                .parse::<u64>()
                .map_err(|e| DisplayError::ConvertToU64Error(e.to_string())),
            self.micro_meter_60
                .parse::<u64>()
                .map_err(|e| DisplayError::ConvertToU64Error(e.to_string())),
            self.micro_meter_180
                .parse::<u64>()
                .map_err(|e| DisplayError::ConvertToU64Error(e.to_string())),
            self.micro_meter_500
                .parse::<u64>()
                .map_err(|e| DisplayError::ConvertToU64Error(e.to_string())),
        ]
        .into_iter()
        .collect::<Result<Vec<u64>, DisplayError>>()
        .map(|vec| {
            ParticleCount::new(
                Uuid::new_v4().to_string(),
                vec[0],
                vec[1],
                vec[2],
                vec[3],
                Utc::now(),
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParticleUI {
    pub new_particle: NewParticle,
    pub particles: Vec<ParticleCount>,
    pub particle: Option<ParticleCount>,
    pub error: Option<String>,
}
