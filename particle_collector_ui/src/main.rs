use std::error::Error;
use std::str::FromStr;

use iced::widget::{button, column, row, text, text_input};
use iced::{executor, Alignment, Application, Command, Element, Settings};
use models::{NewParticleCount, ParticleCount};
use rust_decimal::Decimal;

use reqwest::blocking::Client;

pub fn main() -> iced::Result {
    ParticleUI::run(Settings::default())
}

enum DisplayError {
    Serde(serde_json::Error),
    Reqwest(reqwest::Error),
}

#[derive(Clone, Debug)]
enum Control {
    MicroMeter10(String),
    MicroMeter60(String),
    MicroMeter180(String),
    MicroMeter500(String),
}

#[derive(Clone, Debug, Default)]
struct NewParticle {
    pub micro_meter_10: String,
    pub micro_meter_60: String,
    pub micro_meter_180: String,
    pub micro_meter_500: String,
}

#[derive(Debug, Clone)]
struct ParticleUI {
    new_particle: NewParticle,
    particles: Vec<ParticleCount>,
}

#[derive(Debug, Clone)]
enum Message {
    Loading,
    DisplayData(Vec<ParticleCount>),
    Submit,
    TextChanged(Control),
    DisplayError,
}

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
            Message::Loading => begin_loading(),
            Message::DisplayData(particles) => {
                self.particles = particles;
                Command::none()
            }
            Message::Submit => handle_submit(&self.new_particle),
            Message::TextChanged(control) => {
                match control {
                    Control::MicroMeter10(mm10) => self.new_particle.micro_meter_10 = mm10,
                    Control::MicroMeter60(mm60) => self.new_particle.micro_meter_60 = mm60,
                    Control::MicroMeter180(mm180) => self.new_particle.micro_meter_180 = mm180,
                    Control::MicroMeter500(mm500) => self.new_particle.micro_meter_500 = mm500,
                }
                Command::none()
            }
            Message::DisplayError => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text("Enter micro meters >10"),
                text_input("10 micrometer", &self.new_particle.micro_meter_10)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter10(message)))
            ],
            row![
                text("Enter micro meter >60"),
                text_input("60 micro meter", &self.new_particle.micro_meter_60)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter60(message)))
            ],
            row![
                text("Enter micro meter >180"),
                text_input("180 micro meter", &self.new_particle.micro_meter_180)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter180(message)))
            ],
            row![
                text("Enter micro meter >500"),
                text_input("500 micro meter", &self.new_particle.micro_meter_500)
                    .on_input(|message| Message::TextChanged(Control::MicroMeter500(message)))
            ],
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
        }
    }
}

// TODO: make async
fn begin_loading() -> Command<Message> {
    let data = reqwest::blocking::get("http://localhost:3000/particle")
        .map_err(DisplayError::Reqwest)
        .and_then(|response| response.text().map_err(DisplayError::Reqwest))
        .and_then(|text| {
            serde_json::from_str::<Vec<NewParticleCount>>(&text).map_err(DisplayError::Serde)
        });

    Command::none()
}

// TODO: Make async
fn handle_submit(new_particle: &NewParticle) -> Command<Message> {
    println!("Parsing new particle count data");
    let new_particle = match to_new_particle_counts(new_particle) {
        Ok(new_particle) => new_particle,
        Err(err) => {
            println!("Error while parsing particle data\n{:#?}", err);
            return Command::none();
        }
    };
    println!("Successfully parsed new particle data\n{:#?}", new_particle);

    println!("Serializing body");
    let body = match serde_json::to_string(&new_particle) {
        Ok(body) => body,
        Err(error) => {
            println!("{:#?}", error);
            return Command::none();
        }
    };
    println!("Succesfully serialized body");

    println!("Sending http request");
    // Should be async.
    let text = match Client::new()
        .post("http://localhost:3000/particle")
        .body(body)
        .send()
    {
        Ok(response) => match response.text() {
            Ok(text) => text,
            Err(e) => {
                println!("{:#?}", e);
                return Command::none();
            }
        },
        Err(e) => {
            println!("{:#?}", e);
            return Command::none();
        }
    };
    println!("Finished sending request");

    Command::none()
}

fn to_new_particle_counts(
    new_particle: &NewParticle,
) -> Result<NewParticleCount, rust_decimal::Error> {
    [
        new_particle.micro_meter_10.as_str(),
        new_particle.micro_meter_60.as_str(),
        new_particle.micro_meter_180.as_str(),
        new_particle.micro_meter_500.as_str(),
    ]
    .into_iter()
    .map(Decimal::from_str)
    .collect::<Result<Vec<Decimal>, rust_decimal::Error>>()
    .map(|values| NewParticleCount {
        micro_meter_10: values[0],
        micro_meter_60: values[1],
        micro_meter_180: values[2],
        micro_meter_500: values[3],
    })
}
