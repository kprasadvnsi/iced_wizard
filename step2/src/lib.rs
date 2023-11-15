use iced::widget::{component, text, Component};
use iced::{Element, Renderer};

pub struct Step2 {}

pub fn step2() -> Step2 {
    Step2::new()
}

#[derive(Debug, Clone)]
pub enum Event {}

impl Step2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Message> Component<Message, Renderer> for Step2 {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, _event: Event) -> Option<Message> {
        todo!()
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        text("This is the Step - 2").into()
    }
}

impl<'a, Message> From<Step2> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(step2: Step2) -> Self {
        component(step2)
    }
}
