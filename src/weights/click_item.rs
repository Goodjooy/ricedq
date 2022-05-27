use std::fmt::Debug;

use iced::{button, Button, Element, Renderer};
use iced_native::{event::Status, Shell, Widget};

#[derive(Debug, Clone)]
pub struct State {
    button: button::State,
}

impl State {
    pub fn new() -> Self {
        Self {
            button: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BtnMsg;

pub struct ClickItem<'a, Message, Identify: Copy> {
    inner: Button<'a, BtnMsg>,
    click_id: Identify,
    on_click: Box<dyn Fn(Identify) -> Message + 'a>,
}

impl<'a, Message: Clone + Debug, Identify: Copy> ClickItem<'a, Message, Identify> {
    pub fn new<T, F>(state: &'a mut State, content: T, click_id: Identify, on_click: F) -> Self
    where
        T: Into<Element<'a, BtnMsg>>,
        F: Fn(Identify) -> Message + 'a,
    {
        let button = Button::new(&mut state.button, content)
            .on_press(BtnMsg)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .style(style::Button);
        let fn_box = Box::new(on_click);
        Self {
            inner: button,
            click_id,
            on_click: fn_box,
        }
    }
}

impl<'a, Message, Identify: Copy> Widget<Message, Renderer> for ClickItem<'a, Message, Identify> {
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
        self.inner.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        self.inner
            .draw(renderer, style, layout, cursor_position, viewport)
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

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced_native::event::Status {
        let mut mock_shell_data = Vec::with_capacity(2);
        let mut mock_shell = Shell::new(&mut mock_shell_data);

        if let Status::Captured = self.inner.on_event(
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            &mut mock_shell,
        ) {
            mock_shell_data
                .into_iter()
                .for_each(|_v| shell.publish((self.on_click)(self.click_id)));
            Status::Captured
        } else {
            Status::Ignored
        }
    }
}

impl<'a, Message: 'a, Identify: Copy + 'a> Into<Element<'a, Message>>
    for ClickItem<'a, Message, Identify>
{
    fn into(self) -> Element<'a, Message> {
        Element::new(self)
    }
}

mod style {
    use iced::{button, Color, Vector};

    pub(super) struct Button;

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                shadow_offset: Vector::new(0.0, 0.0),
                background: None,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: Color::TRANSPARENT,
                text_color: Color::BLACK,
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                background: Color::from_rgba(0.5, 0.5, 0.5, 0.5).into(),
                ..self.active()
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                background: Color::from_rgba(0.8, 0.8, 0.8, 0.6).into(),
                ..self.active()
            }
        }
    }
}
