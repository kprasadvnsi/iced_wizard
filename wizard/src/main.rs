use iced::widget::{column, container};
use iced::{Length, Sandbox, Settings};

use step1::step1;
use step2::step2;
use step3::step3;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    Wizard::run(Settings::default())
}

struct Wizard {
    steps: Steps,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for Wizard {
    type Message = Message;

    fn new() -> Self {
        Self {
            steps: Steps::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Iced Wizard")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(column![step1(), step2(), step3(),])
            .padding(20)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

struct Steps {
    pages: Vec<String>,
    current: usize,
}

impl Steps {
    fn new() -> Self {
        Self {
            pages: vec![],
            current: 0,
        }
    }
}
