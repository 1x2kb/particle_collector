pub mod schema;

use chrono::prelude::*;
use diesel::{Insertable, Queryable, Selectable};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, Default, Debug, Clone)]
#[diesel(table_name = crate::schema::particles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewParticleCount {
    #[serde(alias = "mircoMeter10")]
    pub micro_meter_10: Decimal,
    #[serde(alias = "mircoMeter60")]
    pub micro_meter_60: Decimal,
    #[serde(alias = "mircoMeter180")]
    pub micro_meter_180: Decimal,
    #[serde(alias = "mircoMeter500")]
    pub micro_meter_500: Decimal,
}

impl NewParticleCount {
    pub fn new(
        micro_meter_10: Decimal,
        micro_meter_60: Decimal,
        micro_meter_180: Decimal,
        micro_meter_500: Decimal,
    ) -> Self {
        Self {
            micro_meter_10,
            micro_meter_60,
            micro_meter_180,
            micro_meter_500,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::particles)]
pub struct ParticleCount {
    id: i32,
    micro_meter_10: Decimal,
    micro_meter_60: Decimal,
    micro_meter_180: Decimal,
    micro_meter_500: Decimal,
    insert_time: DateTime<Utc>,
}
