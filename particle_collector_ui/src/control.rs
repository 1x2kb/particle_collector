#[derive(Clone, Debug)]
pub enum Control {
    MicroMeter10(String),
    MicroMeter60(String),
    MicroMeter180(String),
    MicroMeter500(String),
}
