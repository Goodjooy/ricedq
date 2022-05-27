use iced::{
    executor, scrollable, Alignment, Application, Color, Column, Command, Container, Row, Rule,
    Text,
};
use iced_native::{subscription, widget::Scrollable, Event};

use crate::{
    weights::{
        click_item::{self, ClickItem},
        msg_edit,
        no_icon_header::NoIconHead,
    },
    FONT,
};

pub struct App {
    state: State,

    info: Vec<Info>,
    users: Vec<String>,
    users_states: Vec<click_item::State>,
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
    Key(Event),
    SelectUser(usize),
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
                users: vec!["哇哈哈".into(), "011好的".into()],
                users_states: vec![click_item::State::new(); 2],
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
            Msg::Key(evn) => match evn {
                Event::Keyboard(evn) => {
                    println!("{:?}", evn)
                }
                _ => {}
            },
            Msg::SelectUser(uid) => println!("Switch to {}", uid),
        };

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events().map(|evn| Msg::Key(evn))
    }
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        self.msg_window.snap_to(1f32);
        match self.state {
            State::NoLogin => Container::new(Text::new("未登录")),
            State::Login => {
                let content = Row::new()
                    .push(
                        self.users
                            .iter()
                            .zip(self.users_states.iter_mut())
                            .enumerate()
                            .map(|(id, (name, state))| {
                                let content = Container::new(
                                    Row::new()
                                        .push(NoIconHead::new(
                                            25,
                                            name.chars().next().unwrap(),
                                            Color::from_rgb(0.2, 0.8, 0.2),
                                        ))
                                        .push(Text::new(name).font(FONT))
                                        .padding(5)
                                        .spacing(15)
                                        .align_items(Alignment::Center),
                                );
                                ClickItem::new(state, content, id, Msg::SelectUser)
                            })
                            .fold(Scrollable::new(&mut self.left), |s, ci| s.push(ci))
                            .width(iced::Length::Units(256))
                            .height(iced::Length::Fill)
                            .spacing(10),
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
