use iced::{Color, Element, Size, Text};
use iced_native::{
    layout,
    renderer::{self, Style},
    Renderer, Widget,
};

use crate::FONT;

pub struct NoIconHead {
    radius: u16,
    inner: Text,
    color: Color,
}

impl NoIconHead {
    pub fn new(radius: u16, name: char, color: Color) -> Self {
        let r = radius as f64;
        let size = r * 2.0f64.powf(0.5) * 0.75;
        Self {
            radius,
            inner: Text::new(name).font(FONT).size(size as u16),
            color,
        }
    }
}

impl<Message> Widget<Message, iced::Renderer> for NoIconHead {
    fn width(&self) -> iced::Length {
        iced::Length::Shrink
    }

    fn height(&self) -> iced::Length {
        iced::Length::Shrink
    }

    fn layout(
        &self,
        renderer: &iced::Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let size = Size::new(self.radius as f32 * 2.0, self.radius as f32 * 2.0);

        let mut inner = Widget::<Message, _>::layout(&self.inner, renderer, limits);

        inner.align(
            iced::Alignment::Center,
            iced::Alignment::Center,
            size.clone(),
        );

        let parent = layout::Node::with_children(size, vec![inner]);

        parent
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        _style: &Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: self.radius as f32,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            self.color,
        );

        Widget::<Message, _>::draw(
            &self.inner,
            renderer,
            &Style {
                text_color: Color::WHITE,
            },
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
        );
    }
}

impl<'a, Message> Into<Element<'a, Message>> for NoIconHead
where
    Self: 'a,
{
    fn into(self) -> Element<'a, Message> {
        Element::new(self)
    }
}
