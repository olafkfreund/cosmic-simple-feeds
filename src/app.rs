use cosmic::app::{Core, Task};
use cosmic::iced::{Alignment, Length, window::Id, time, Subscription};
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{button, container, scrollable, text, Column};
use cosmic::iced::widget::text_input;
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::Element;
use std::time::Duration;
use rss::Channel;

/// O modelo principal da aplicação
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

/// Estado de carregamento
#[derive(Debug, Clone)]
pub enum State {
    Loading,
    Loaded,
    Error(String),
}

/// Mensagens que a aplicação pode receber
#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    FeedLoaded(Result<Channel, String>),
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

        let app = AppModel {
            core,
            popup: None,
            config: flags.clone(),
            cosmic_cfg,
            items: Vec::new(),
            state: State::Loading,
            editing: false,
            new_feed: String::new(),
        };

        // start fetching the first feed if available; subscription will handle periodic refreshes
        let mut tasks = Vec::new();
        if let Some(url) = app.config.feeds.get(0).cloned() {
            tasks.push(fetch_feed(url));
        }

        let task = if tasks.is_empty() { Task::none() } else { cosmic::task::batch(tasks) };
        (app, task)
    }

    fn subscription(&self) -> Subscription<Message> {
        let secs = self.config.refresh_interval_seconds.max(10);
        time::every(Duration::from_secs(secs)).map(|_| Message::Refresh)
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
                return Task::none();
            }
            Message::Refresh => {
                self.state = State::Loading;
                if let Some(url) = self.config.feeds.get(0).cloned() {
                    fetch_feed(url)
                } else {
                    Task::none()
                }
            }
            Message::FeedLoaded(res) => {
                match res {
                    Ok(channel) => {
                        println!("Feed carregado com sucesso: {} itens", channel.items().len());
                        self.items = channel.items().to_vec();
                        self.state = State::Loaded;
                    }
                    Err(e) => {
                        println!("Erro no feed: {}", e);
                        self.state = State::Error(e);
                    }
                }
                Task::none()
            }
            Message::OpenLink(url) => {
                    // Open the link, then close the popup if open to avoid stale id
                    let _ = open::that(&url);
                    if let Some(id) = self.popup.take() {
                        return cosmic::task::message(cosmic::Action::Cosmic(
                            cosmic::app::Action::Surface(destroy_popup(id)),
                        ));
                    }
                    Task::none()
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
                    self.config.feeds.push(s.to_string());
                    self.new_feed.clear();
                    // persist
                    if let Some(cfg) = &self.cosmic_cfg {
                        let _ = self.config.set_feeds(cfg, self.config.feeds.clone());
                    }
                    // refresh from the first feed
                    if let Some(url) = self.config.feeds.get(0).cloned() {
                        self.state = State::Loading;
                        return fetch_feed(url);
                    }
                }
                Task::none()
            }
            Message::RemoveFeed(idx) => {
                if idx < self.config.feeds.len() {
                    self.config.feeds.remove(idx);
                    if let Some(cfg) = &self.cosmic_cfg {
                        let _ = self.config.set_feeds(cfg, self.config.feeds.clone());
                    }
                    // ensure items reflect first feed
                    if let Some(url) = self.config.feeds.get(0).cloned() {
                        self.state = State::Loading;
                        return fetch_feed(url);
                    } else {
                        self.items.clear();
                        self.state = State::Loaded;
                    }
                }
                Task::none()
            }
            Message::Surface(a) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ));
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let have_popup = self.popup;

        // build a text button that toggles the popup (show 'News' instead of an icon)
        let label = self.core.applet.text("News");
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
                            // request a slightly taller but fixed-width popup for better layout
                            let mut popup_settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                new_id,
                                Some((280, 320)),
                                None,
                                None,
                            );

                            popup_settings.positioner.anchor_rect = cosmic::iced::Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };
                                popup_settings.positioner.anchor_rect = cosmic::iced::Rectangle {
                                    x: (bounds.x - offset.x) as i32,
                                    y: (bounds.y - offset.y) as i32,
                                    width: bounds.width as i32,
                                    height: bounds.height as i32,
                                };

                            popup_settings
                        },
                        Some(Box::new(move |state: &AppModel| {
                            // build popup content from current state
                            let title = text("Meus Feeds RSS")
                                .size(18)
                                .width(Length::Fill)
                                .align_x(Alignment::Center);

                            let mut list = Column::new().spacing(6).padding([6,8]);

                            // manage button row
                            let manage_row = button::text(if state.editing { "Feito" } else { "Gerenciar" })
                                .on_press(Message::ToggleManage)
                                .padding(6);

                            list = list.push(container(Column::new().push(title).push(manage_row)).padding(4));

                            // when editing, show feeds list and add input
                            if state.editing {
                                let mut feeds_col = Column::new().spacing(6).padding([6,4]);
                                for (i, f) in state.config.feeds.iter().enumerate() {
                                    let row = Column::new()
                                        .spacing(2)
                                        .push(text(f).size(13))
                                        .push(button::text("Remover").on_press(Message::RemoveFeed(i)).padding(4));
                                    feeds_col = feeds_col.push(row);
                                }

                                // add input
                                let input = text_input("Nova URL de feed", &state.new_feed)
                                    .on_input(Message::FeedInputChanged)
                                    .on_submit(Message::AddFeed)
                                    .width(Length::Fill)
                                    .padding(6);

                                let add_row = Column::new().spacing(6).push(input).push(button::text("Adicionar").on_press(Message::AddFeed).padding(6));

                                list = list.push(feeds_col).push(add_row);
                            }
                            else {
                                // normal view (not editing)
                                match &state.state {
                                    State::Loading => {
                                        list = list.push(text("Carregando notícias..."));
                                    }
                                    State::Error(e) => {
                                        list = list.push(text(format!("Erro: {}", e)));
                                    }
                                    State::Loaded => {
                                        if state.items.is_empty() {
                                            list = list.push(text("Nenhum item encontrado."));
                                        } else {
                                            for item in state.items.iter().take(8) {
                                                let item_title = item.title().unwrap_or("Sem título");
                                                let link = item.link().unwrap_or("").to_string();
                                                let date = item.pub_date().unwrap_or("");

                                                let row = Column::new()
                                                    .spacing(2)
                                                    .push(
                                                        button::text(item_title)
                                                            .on_press(Message::OpenLink(link.clone()))
                                                            .width(Length::Fill)
                                                            .padding(6),
                                                    )
                                                    .push(text(date).size(12));

                                                list = list.push(row);
                                            }
                                        }
                                    }
                                }
                            }

                            let content = state.core.applet.popup_container(Column::new().push(scrollable(list)));

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

    fn view_window(&self, _id: Id) -> Element<Message> {
        // Not used for applet popups
        container(text("")).width(Length::Fixed(0.)).height(Length::Fixed(0.)).into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}

/// Função auxiliar assíncrona para baixar o feed
fn fetch_feed(url: String) -> Task<Message> {
    Task::perform(
        async move {
            let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
            let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
            let channel = Channel::read_from(&bytes[..]).map_err(|e| e.to_string())?;
            Ok(channel)
        },
        |res| Message::FeedLoaded(res).into(),
    )
}