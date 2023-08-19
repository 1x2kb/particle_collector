use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct NewParticleCount {
    #[serde(alias = "mircoMeter10")]
    pub micro_meter_10: u64,
    #[serde(alias = "mircoMeter60")]
    pub micro_meter_60: u64,
    #[serde(alias = "mircoMeter180")]
    pub micro_meter_180: u64,
    #[serde(alias = "mircoMeter500")]
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

#[derive(Debug, Clone)]
pub enum DisplayError {
    Serde(String),
    NumParseError(String),
    FileReadError(String),
    U8parseError(String),
}
