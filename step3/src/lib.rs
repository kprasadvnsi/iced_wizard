use iced::widget::{component, text, Component};
use iced::{Element, Renderer};

pub struct Step3 {}

pub fn step3() -> Step3 {
    Step3::new()
}

#[derive(Debug, Clone)]
pub enum Event {}

impl Step3 {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Message> Component<Message, Renderer> for Step3 {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, _event: Event) -> Option<Message> {
        todo!()
    }

    fn view(&self, _state: &Self::State) -> Element<Event, Renderer> {
        text("This is the Step - 3").into()
    }
}

impl<'a, Message> From<Step3> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(step3: Step3) -> Self {
        component(step3)
    }
}
