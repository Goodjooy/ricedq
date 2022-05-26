use iced::{button, text_input, Button, Element, Renderer, Row, Text, TextInput};
use iced_native::{event::Status, renderer, Shell, Widget};

use crate::FONT;

pub struct MessageEdit<'a, Message> {
    // inner weight
    inner: Row<'a, Msg>,
    on_send: Box<dyn Fn(String) -> Message + 'a>,
    input: &'a mut Option<String>,
}

impl<'a, Message> MessageEdit<'a, Message> {
    pub fn new(state: &'a mut State, handle: impl Fn(String) -> Message + 'a) -> Self {
        Self {
            inner: Row::new()
                .align_items(iced::Alignment::Fill)
                .height(iced::Length::Shrink)
                .spacing(5)
                .push(
                    TextInput::new(
                        &mut state.text_input,
                        "your message",
                        match &state.input {
                            Some(s) => s.as_str(),
                            None => "",
                        },
                        Msg::OnEdit,
                    )
                    .width(iced::Length::FillPortion(9))
                    .font(FONT)
                    .on_submit(Msg::Send),
                )
                .push(
                    Button::new(
                        &mut state.send_button,
                        Text::new("发送")
                            .font(FONT)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .on_press(Msg::Send)
                    .width(iced::Length::FillPortion(1)),
                ),
            on_send: Box::new(handle),
            input: &mut state.input,
        }
    }
}

pub struct State {
    input: Option<String>,
    text_input: text_input::State,
    send_button: button::State,
}

impl State {
    pub fn new() -> Self {
        Self {
            input: None,
            text_input: text_input::State::new(),
            send_button: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    OnEdit(String),
    Send,
}

impl<'a, Message> Widget<Message, Renderer> for MessageEdit<'a, Message> {
    fn width(&self) -> iced::Length {
        Widget::width(&self.inner)
    }

    fn height(&self) -> iced::Length {
        Widget::height(&self.inner)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        Widget::layout(&self.inner, renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        Widget::draw(
            &self.inner,
            renderer,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> Status {
        let mut temp_shell_inner = Vec::<Msg>::with_capacity(2);
        let mut temp_shell = Shell::new(&mut temp_shell_inner);

        let resp = self.inner.on_event(
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            &mut temp_shell,
        );

        match resp {
            Status::Ignored => Status::Ignored,
            Status::Captured => {
                let msg = temp_shell_inner.into_iter().next();
                if let Some(msg) = msg {
                    match msg {
                        Msg::OnEdit(s) => {
                            self.input.replace(s);
                            Status::Captured
                        }
                        Msg::Send => match self.input.take() {
                            Some(s) => {
                                let msg = (self.on_send)(s);
                                shell.publish(msg);
                                Status::Captured
                            }
                            None => Status::Captured,
                        },
                    }
                } else {
                    Status::Captured
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        self.inner
            .mouse_interaction(layout, cursor_position, viewport, renderer)
    }

}

impl<'a, Message> Into<Element<'a, Message>> for MessageEdit<'a, Message>
where
    Self: 'a,
{
    fn into(self) -> Element<'a, Message> {
        Element::new(self)
    }
}
