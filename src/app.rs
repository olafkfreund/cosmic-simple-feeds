use cosmic::app::{Core, Task};
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::iced::{Alignment, Length, window::Id, time, Subscription};
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{button, container, scrollable, text, text_input, Column};
use cosmic::Element;
use std::time::Duration;

use crate::error::FeedError;
use crate::fl;

struct ConfigSubscriptionId;

pub struct AppModel {
    core: Core,
    popup: Option<Id>,
    config: crate::config::Config,
    cosmic_cfg: Option<cosmic::cosmic_config::Config>,
    items: Vec<rss::Item>,
    state: State,
    editing: bool,
    new_feed: String,
}

#[derive(Debug, Clone)]
pub enum State {
    Loading,
    Loaded,
    Error(FeedError),
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    FeedsLoaded(Result<Vec<rss::Item>, FeedError>),
    ConfigChanged(crate::config::Config),
    OpenLink(String),
    PopupClosed(Id),
    Surface(cosmic::surface::Action),
    ToggleManage,
    FeedInputChanged(String),
    AddFeed,
    RemoveFeed(usize),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = crate::config::Config;
    type Message = Message;

    const APP_ID: &'static str = "com.github.marcossl10.cosmic-simple-feeds";

    fn init(core: Core, flags: crate::config::Config) -> (Self, Task<Message>) {
        let cosmic_cfg = cosmic::cosmic_config::Config::new(
            AppModel::APP_ID,
            crate::config::Config::VERSION,
        )
        .ok();

        let feeds = flags.feeds.clone();
        let app = AppModel {
            core,
            popup: None,
            config: flags,
            cosmic_cfg,
            items: Vec::new(),
            state: State::Loading,
            editing: false,
            new_feed: String::new(),
        };

        (app, fetch_all_feeds(feeds))
    }

    fn subscription(&self) -> Subscription<Message> {
        let secs = self.config.refresh_interval_seconds.max(10);
        Subscription::batch(vec![
            time::every(Duration::from_secs(secs)).map(|_| Message::Refresh),
            cosmic::cosmic_config::config_subscription::<_, crate::config::Config>(
                std::any::TypeId::of::<ConfigSubscriptionId>(),
                Self::APP_ID.into(),
                crate::config::Config::VERSION,
            )
            .map(|update| {
                for err in &update.errors {
                    eprintln!("config error: {err}");
                }
                Message::ConfigChanged(update.config)
            }),
        ])
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
                Task::none()
            }
            Message::Refresh => {
                // Don't set Loading state on periodic refresh — keep old items visible
                if self.items.is_empty() {
                    self.state = State::Loading;
                }
                fetch_all_feeds(self.config.feeds.clone())
            }
            Message::ConfigChanged(new_config) => {
                if new_config != self.config {
                    let feeds_changed = new_config.feeds != self.config.feeds;
                    self.config = new_config;
                    if feeds_changed {
                        self.state = State::Loading;
                        return fetch_all_feeds(self.config.feeds.clone());
                    }
                }
                Task::none()
            }
            Message::FeedsLoaded(res) => {
                match res {
                    Ok(items) => {
                        self.items = items;
                        self.state = State::Loaded;
                    }
                    Err(e) => {
                        eprintln!("feed error: {e}");
                        // Keep old items visible on refresh errors
                        if self.items.is_empty() {
                            self.state = State::Error(e);
                        }
                    }
                }
                Task::none()
            }
            Message::OpenLink(url) => {
                if let Err(e) = crate::feed::validate_url(&url) {
                    eprintln!("refusing to open invalid URL: {e}");
                    return Task::none();
                }
                let _ = open::that(&url);
                if let Some(id) = self.popup.take() {
                    cosmic::task::message(cosmic::Action::Cosmic(
                        cosmic::app::Action::Surface(destroy_popup(id)),
                    ))
                } else {
                    Task::none()
                }
            }
            Message::ToggleManage => {
                self.editing = !self.editing;
                Task::none()
            }
            Message::FeedInputChanged(s) => {
                self.new_feed = s;
                Task::none()
            }
            Message::AddFeed => {
                let s = self.new_feed.trim();
                if !s.is_empty() {
                    let mut url = if s.starts_with("http://") || s.starts_with("https://") {
                        s.to_string()
                    } else {
                        format!("https://{}", s)
                    };

                    if url.contains("news.google.com") && !url.contains("/rss/") {
                        url = url.replace("news.google.com/", "news.google.com/rss/");
                    }

                    if let Err(e) = crate::feed::validate_url(&url) {
                        eprintln!("invalid feed URL: {e}");
                        return Task::none();
                    }

                    self.config.feeds.push(url);
                    self.new_feed.clear();
                    if let Some(cfg) = &self.cosmic_cfg {
                        let _ = self.config.set_feeds(cfg, self.config.feeds.clone());
                    }
                    self.state = State::Loading;
                    return fetch_all_feeds(self.config.feeds.clone());
                }
                Task::none()
            }
            Message::RemoveFeed(idx) => {
                if idx < self.config.feeds.len() {
                    self.config.feeds.remove(idx);
                    if let Some(cfg) = &self.cosmic_cfg {
                        let _ = self.config.set_feeds(cfg, self.config.feeds.clone());
                    }
                    if self.config.feeds.is_empty() {
                        self.items.clear();
                        self.state = State::Loaded;
                    } else {
                        self.state = State::Loading;
                        return fetch_all_feeds(self.config.feeds.clone());
                    }
                }
                Task::none()
            }
            Message::Surface(a) => {
                cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ))
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let have_popup = self.popup;

        let label = self.core.applet.text(fl!("applet-label"));
        let btn = self
            .core
            .applet
            .button_from_element(label, true)
            .on_press_with_rectangle(move |offset, bounds| {
                if let Some(id) = have_popup {
                    Message::Surface(destroy_popup(id))
                } else {
                    Message::Surface(app_popup::<AppModel>(
                        move |state: &mut AppModel| {
                            let new_id = Id::unique();
                            state.popup = Some(new_id);
                            let mut popup_settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                new_id,
                                Some((360, 400)),
                                None,
                                None,
                            );

                            popup_settings.positioner.anchor_rect = cosmic::iced::Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };

                            popup_settings
                        },
                        Some(Box::new(move |state: &AppModel| {
                            let sp = cosmic::theme::active().cosmic().spacing;

                            let title = text(fl!("popup-title"))
                                .size(sp.space_l)
                                .width(Length::Fill)
                                .align_x(Alignment::Center);

                            let mut list = Column::new()
                                .spacing(sp.space_xs)
                                .padding([sp.space_xs, sp.space_s]);

                            let manage_row = button::text(if state.editing { fl!("done") } else { fl!("manage") })
                                .on_press(Message::ToggleManage)
                                .padding(sp.space_xs);

                            list = list.push(
                                container(Column::new().push(title).push(manage_row))
                                    .padding(sp.space_xs),
                            );

                            if state.editing {
                                let mut feeds_col = Column::new()
                                    .spacing(sp.space_xs)
                                    .padding([sp.space_xs, sp.space_xxs]);
                                for (i, f) in state.config.feeds.iter().enumerate() {
                                    let row = Column::new()
                                        .spacing(sp.space_xxs)
                                        .push(text(f).size(sp.space_m))
                                        .push(
                                            button::text(fl!("remove"))
                                                .on_press(Message::RemoveFeed(i))
                                                .padding(sp.space_xxs),
                                        );
                                    feeds_col = feeds_col.push(row);
                                }

                                let input = text_input(fl!("feed-input-placeholder"), &state.new_feed)
                                    .on_input(Message::FeedInputChanged)
                                    .on_submit(|_| Message::AddFeed)
                                    .width(Length::Fill)
                                    .padding(sp.space_xs);

                                let add_row = Column::new()
                                    .spacing(sp.space_xs)
                                    .push(input)
                                    .push(
                                        button::text(fl!("add"))
                                            .on_press(Message::AddFeed)
                                            .padding(sp.space_xs),
                                    );

                                list = list.push(feeds_col).push(add_row);
                            } else {
                                match &state.state {
                                    State::Loading => {
                                        list = list.push(text(fl!("loading")));
                                    }
                                    State::Error(e) => {
                                        list = list.push(text(fl!("error", message = e.to_string())));
                                    }
                                    State::Loaded => {
                                        if state.items.is_empty() {
                                            list = list.push(text(fl!("no-items")));
                                        } else {
                                            for item in state.items.iter().take(10) {
                                                let raw_title = item.title().unwrap_or_default();
                                                let item_title = if raw_title.is_empty() {
                                                    fl!("untitled")
                                                } else {
                                                    crate::feed::sanitize_text(raw_title).into_owned()
                                                };
                                                let link = item.link().unwrap_or("").to_string();
                                                let date = crate::feed::sanitize_text(
                                                    item.pub_date().unwrap_or(""),
                                                );

                                                let row = Column::new()
                                                    .spacing(sp.space_xxs)
                                                    .push(
                                                        button::text(item_title)
                                                            .on_press(Message::OpenLink(link.clone()))
                                                            .width(Length::Fill)
                                                            .padding(sp.space_xs),
                                                    )
                                                    .push(text(date).size(sp.space_m));

                                                list = list.push(row);
                                            }
                                        }
                                    }
                                }
                            }

                            let content = state.core.applet.popup_container(
                                Column::new().push(scrollable(list)),
                            );

                            Element::from(content).map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        Element::from(self.core.applet.applet_tooltip::<Message>(
            btn,
            "Feeds",
            self.popup.is_some(),
            |a| Message::Surface(a),
            None,
        ))
    }

    fn view_window(&self, _id: Id) -> Element<'_, Message> {
        container(text("")).width(Length::Fixed(0.)).height(Length::Fixed(0.)).into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}

fn fetch_all_feeds(urls: Vec<String>) -> Task<Message> {
    Task::perform(crate::feed::fetch_all(urls), |res| {
        Message::FeedsLoaded(res).into()
    })
}
