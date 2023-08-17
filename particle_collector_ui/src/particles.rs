use models::ParticleCount;

#[derive(Clone, Debug, Default)]
pub struct NewParticle {
    pub micro_meter_10: String,
    pub micro_meter_60: String,
    pub micro_meter_180: String,
    pub micro_meter_500: String,
}

#[derive(Debug, Clone)]
pub struct ParticleUI {
    pub new_particle: NewParticle,
    pub particles: Vec<ParticleCount>,
    pub particle: Option<ParticleCount>,
    pub error: Option<String>,
}
