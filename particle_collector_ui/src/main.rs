mod control;
mod message;
mod particles;

use std::num::ParseIntError;

use control::Control;
use iced::widget::{button, column, row, text, text_input};
use iced::{executor, Alignment, Application, Command, Element, Settings};
use message::Message;
use models::{DisplayError, NewParticleCount, ParticleCount};
use particles::{NewParticle, ParticleUI};

impl Application for ParticleUI {
    type Message = Message;
    type Executor = executor::Default;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (
            ParticleUI {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Particle Counter")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::_Loading => begin_loading(),
            Message::DisplayData(particles) => {
                self.particles = particles;
                Command::none()
            }
            Message::Submit => handle_submit(&self.new_particle),
            Message::SuccessfulWrite(particle_count) => {
                self.particle = particle_count;
                self.error = None;
                Command::none()
            }
            Message::TextChanged(control) => {
                match control {
                    Control::MicroMeter10(mm10) => self.new_particle.micro_meter_10 = mm10,
                    Control::MicroMeter60(mm60) => self.new_particle.micro_meter_60 = mm60,
                    Control::MicroMeter180(mm180) => self.new_particle.micro_meter_180 = mm180,
                    Control::MicroMeter500(mm500) => self.new_particle.micro_meter_500 = mm500,
                }
                Command::none()
            }
            Message::Error(display_error) => {
                println!("Ran Message::Error");
                self.particle = None;
                self.error = display_error.map(|error| match error {
                    DisplayError::Serde(error) => error,
                    DisplayError::NumParseError(error) => error,
                    DisplayError::FileReadError(e) => e,
                    DisplayError::U8parseError(e) => e,
                    DisplayError::WriteError(e) => e,
                    DisplayError::ConvertToU64Error(e) => e,
                });
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text("Enter micro meters >10"),
                text_input("10 micrometer", &self.new_particle.micro_meter_10)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter10(message)))
            ]
            .align_items(Alignment::Center),
            row![
                text("Enter micro meter >60"),
                text_input("60 micro meter", &self.new_particle.micro_meter_60)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter60(message)))
            ]
            .align_items(Alignment::Center),
            row![
                text("Enter micro meter >180"),
                text_input("180 micro meter", &self.new_particle.micro_meter_180)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter180(message)))
            ]
            .align_items(Alignment::Center),
            row![
                text("Enter micro meter >500"),
                text_input("500 micro meter", &self.new_particle.micro_meter_500)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter500(message)))
            ],
            row![text(match &self.error {
                Some(error) => error,
                None => "",
            })],
            row![text(match &self.particle {
                Some(_) => "Wrote particle",
                None => "",
            })]
            .align_items(Alignment::Center),
            button("Submit").on_press(Message::Submit)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

impl Default for ParticleUI {
    fn default() -> Self {
        Self {
            new_particle: NewParticle {
                ..Default::default()
            },
            particles: Vec::default(),
            particle: None,
            error: None,
        }
    }
}

fn begin_loading() -> Command<Message> {
    Command::perform(get_data(), |data| match data {
        Ok(d) => Message::DisplayData(d),
        Err(error) => Message::Error(Some(error)),
    })
}

async fn get_data() -> Result<Vec<ParticleCount>, DisplayError> {
    file_operations::parse_data("./data/data.csv")
}

fn handle_submit(new_particle: &NewParticle) -> Command<Message> {
    Command::perform(
        write_data(new_particle.clone()),
        |write_result| match write_result {
            Ok(value) => Message::SuccessfulWrite(Some(value)),
            Err(e) => Message::Error(Some(e)),
        },
    )
}

async fn write_data(particle_data: NewParticle) -> Result<ParticleCount, DisplayError> {
    let _new_particle = to_new_particle_counts(&particle_data)?;
    println!("Successfully converted input into particle type");

    // Ok(ParticleCount::new(
    //     Uuid::new_v4().to_string(),
    //     150000u64,
    //     25000u64,
    //     12500u64,
    //     7000u64,
    //     Utc::now(),
    // ))

    let particle_count: Result<ParticleCount, DisplayError> = particle_data.into();

    particle_count.and_then(|particle_data| {
        file_operations::write_data("", &particle_data).map(|_| particle_data)
    })
}

fn to_new_particle_counts(new_particle: &NewParticle) -> Result<NewParticleCount, DisplayError> {
    [
        new_particle.micro_meter_10.as_str(),
        new_particle.micro_meter_60.as_str(),
        new_particle.micro_meter_180.as_str(),
        new_particle.micro_meter_500.as_str(),
    ]
    .into_iter()
    .map(|value| value.parse::<u64>())
    .collect::<Result<Vec<u64>, ParseIntError>>()
    .map(|values| NewParticleCount {
        micro_meter_10: values[0],
        micro_meter_60: values[1],
        micro_meter_180: values[2],
        micro_meter_500: values[3],
    })
    .map_err(|error| DisplayError::NumParseError(error.to_string()))
}

pub fn main() -> iced::Result {
    ParticleUI::run(Settings::default())
}
