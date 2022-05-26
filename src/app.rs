use iced::{
    executor, scrollable, Alignment, Application, Color, Column, Command, Container, Row, Rule,
    Subscription, Text,
};
use iced_native::widget::Scrollable;

use crate::{
    weights::{msg_edit, no_icon_header::NoIconHead},
    FONT,
};

pub struct App {
    state: State,

    info: Vec<Info>,
    self_id: i64,

    left: scrollable::State,
    msg_window: scrollable::State,

    msg_editor: msg_edit::State,
}

pub enum State {
    NoLogin,
    Login,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Send(String),
}

#[derive(Debug, Clone)]
pub struct Info {
    uin: i64,
    contain: String,
}

#[derive(Debug)]
pub struct Cfg {
    device: Option<ricq::device::Device>,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Msg;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                state: State::Login,
                left: scrollable::State::new(),
                msg_window: scrollable::State::new(),
                info: vec![Info {
                    uin: 114514,
                    contain: String::from("好耶"),
                }],
                self_id: 1919,
                msg_editor: msg_edit::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RICEDQ".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Msg::Send(s) => {
                self.info.push(Info {
                    uin: 1919,
                    contain: s,
                });
            }
        };

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::none()
    }
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match self.state {
            State::NoLogin => Container::new(Text::new("未登录")),
            State::Login => {
                let content = Row::new()
                    .push(
                        Scrollable::new(&mut self.left)
                            .width(iced::Length::Units(256))
                            .height(iced::Length::Fill)
                            .spacing(10)
                            .push(
                                Container::new(
                                    Row::new()
                                        .push(NoIconHead::new(
                                            25,
                                            "好耶".chars().next().unwrap(),
                                            Color::from_rgb(0.8, 0.2, 0.2),
                                        ))
                                        .push(Text::new("好耶").font(FONT))
                                        .padding(5)
                                        .spacing(15)
                                        .align_items(Alignment::Center),
                                )
                                .align_x(iced::alignment::Horizontal::Left)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(iced::Length::Fill)
                                .height(iced::Length::Units(50)),
                            )
                            .push(
                                Container::new(
                                    Row::new()
                                        .push(NoIconHead::new(
                                            25,
                                            "+好耶".chars().next().unwrap(),
                                            Color::from_rgb(0.2, 0.8, 0.2),
                                        ))
                                        .push(Text::new("+好耶").font(FONT))
                                        .padding(5)
                                        .spacing(15)
                                        .align_items(Alignment::Center),
                                )
                                .align_x(iced::alignment::Horizontal::Left)
                                .align_y(iced::alignment::Vertical::Center)
                                .width(iced::Length::Fill)
                                .height(iced::Length::Units(50)),
                            )
                            ,
                    )
                    .push(Rule::vertical(30))
                    .push(
                        Column::new()
                            .align_items(iced::Alignment::Fill)
                            .padding(5)
                            .spacing(5)
                            .push(
                                self.info
                                    .iter()
                                    .cloned()
                                    .map(|Info { uin, contain }| {
                                        Column::new()
                                            .push(Text::new(uin.to_string()).font(FONT).size(12))
                                            .push(Container::new(
                                                Text::new(contain).font(FONT).size(25),
                                            ))
                                            .padding(15)
                                            .align_items(if uin == self.self_id {
                                                Alignment::End
                                            } else {
                                                Alignment::Start
                                            })
                                            .width(iced::Length::Fill)
                                    })
                                    .fold(Scrollable::new(&mut self.msg_window), |s, c| s.push(c))
                                    .align_items(iced::Alignment::Start)
                                    .height(iced::Length::Fill),
                            )
                            .push(Rule::horizontal(5))
                            .push(msg_edit::MessageEdit::new(&mut self.msg_editor, Msg::Send)),
                    );

                Container::new(content)
            }
        }
        .into()
    }
}
