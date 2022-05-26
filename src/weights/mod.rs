pub mod no_icon_header;
pub mod msg_edit;
use iced::{Command, Element, Subscription};

pub trait SubWidget<'a> {
    type Message;
    type State;
    type Args;

    fn new(state: & 'a mut Self::State, args: Self::Args) -> Self;

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message>;

    fn view(&mut self) -> Element<'_, Self::Message>;

    fn subscription(&self) -> Subscription<Self::Message>;
}
