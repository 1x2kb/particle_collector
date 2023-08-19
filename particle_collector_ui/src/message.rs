use models::ParticleCount;

use crate::{control::Control, DisplayError};

#[derive(Debug, Clone)]
pub enum Message {
    _Loading,
    DisplayData(Vec<ParticleCount>),
    Submit,
    SuccessfulWrite(Option<ParticleCount>),
    TextChanged(Control),
    Error(Option<DisplayError>),
}
