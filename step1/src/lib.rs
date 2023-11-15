use iced::widget::{component, text, Component};
use iced::{Element, Renderer};

pub struct Step1 {}

pub fn step1() -> Step1 {
    Step1::new()
}

#[derive(Debug, Clone)]
pub enum Event {}

impl Step1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Message> Component<Message, Renderer> for Step1 {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, _event: Event) -> Option<Message> {
        todo!()
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        text("This is the Step - 1").into()
    }
}

impl<'a, Message> From<Step1> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(step1: Step1) -> Self {
        component(step1)
    }
}
