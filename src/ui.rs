use iced::widget::{self, column, container, image, row, text};
use iced::{Alignment, Application, Command, Element, Length, Theme};

use crate::structs::{ConfigFile, GirlGeniusPage};
use crate::{next, parse_gg_string_for_date, previous};

#[derive(Debug)]
pub enum GggUi {
    Loading,
    Loaded { image: UiPage },
    Errored,
}

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Prev,
    Init,
    Loaded(Result<UiPage, Error>),
}

impl Application for GggUi {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (GggUi, Command<Message>) {
        (
            GggUi::Loading,
            Command::perform(UiPage::init(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            GggUi::Loading => "Loading...",
            GggUi::Loaded { image, .. } => &image.date,
            GggUi::Errored => "Error",
        };
        format!("GggUi - {}", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok(image)) => {
                *self = GggUi::Loaded { image };

                Command::none()
            }
            Message::Loaded(Err(_err)) => {
                *self = GggUi::Errored;
                Command::none()
            }
            Message::Next => match self {
                GggUi::Loading => Command::none(),
                _ => {
                    *self = GggUi::Loading;
                    Command::perform(UiPage::next(), Message::Loaded)
                }
            },
            Message::Prev => match self {
                GggUi::Loading => Command::none(),
                _ => {
                    *self = GggUi::Loading;
                    Command::perform(UiPage::prev(), Message::Loaded)
                }
            },
            Message::Init => match self {
                GggUi::Loading => Command::none(),
                _ => {
                    *self = GggUi::Loading;
                    Command::perform(UiPage::init(), Message::Loaded)
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            GggUi::Loading => column![text("Loading page...").size(40),].width(Length::Shrink),
            GggUi::Loaded { image } => column![
                image.view(),
                row![
                    button("Prev").on_press(Message::Prev),
                    button("Next").on_press(Message::Next),
                ]
                .align_items(Alignment::Center)
            ]
            .spacing(20)
            .align_items(Alignment::Center),
            GggUi::Errored => column![
                text("something broke").size(40),
                button("try to load the current page").on_press(Message::Init)
            ]
            .spacing(20)
            .align_items(Alignment::Center),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct UiPage {
    image: image::Handle,
    date: String,
}

impl UiPage {
    fn view(&self) -> Element<Message> {
        row![image::viewer(self.image.clone())]
            .spacing(20)
            .align_items(Alignment::Center)
            // .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    async fn prev() -> Result<UiPage, Error> {
        let conf = ConfigFile {
            path: std::env::var("ggg_config_path").unwrap(),
        };

        if conf.read().latest_page == String::from("20021104") {
            Ok(UiPage {
                image: image::Handle::from_path(format!(
                    "{}/20021104.jpg",
                    std::env::var("ggg_cache_path").unwrap()
                )),
                date: "2022 11 04".to_string(),
            })
        } else {
            let prev = previous(conf.clone(), &std::env::var("ggg_cache_path").unwrap()).await;

            let image = match prev {
                Some((_, path)) => image::Handle::from_path(path),
                None => image::Handle::from_path(
                    GirlGeniusPage::new(parse_gg_string_for_date(conf.clone().read().latest_page))
                        .await
                        .save(&std::env::var("ggg_cache_path").unwrap())
                        .await,
                ),
            };

            Ok(UiPage {
                image,
                date: "fuck you this is a banana not a dried plum".to_string(),
            })
        }
    }
    async fn next() -> Result<UiPage, Error> {
        let image = image::Handle::from_path(
            next(
                ConfigFile {
                    path: std::env::var("ggg_config_path").unwrap(),
                },
                &std::env::var("ggg_cache_path").unwrap(),
            )
            .await
            .unwrap()
            .1,
        );

        Ok(UiPage {
            image,
            date: "fuck you this is a cliff not a dried plum".to_string(),
        })
    }
    async fn init() -> Result<UiPage, Error> {
        let image = image::Handle::from_path(
            GirlGeniusPage::new(parse_gg_string_for_date(
                ConfigFile {
                    path: std::env::var("ggg_config_path").unwrap(),
                }
                .read()
                .latest_page,
            ))
            .await
            .save(&std::env::var("ggg_cache_path").unwrap())
            .await,
        );

        Ok(UiPage {
            image,
            date: "fuck you this is a lights not a dried plum".to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Error {}

fn button(text: &str) -> widget::Button<'_, Message> {
    widget::button(text).padding(10)
}
