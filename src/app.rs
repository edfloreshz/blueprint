// SPDX-License-Identifier: GPL-3.0

use crate::config::Config;
use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Alignment, Subscription};
use cosmic::widget::{self, icon, menu, nav_bar};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element};
use futures_util::SinkExt;
use models::package::{Package, Source};
use page::PageView;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod models;
pub mod page;

const REPOSITORY: &str = "https://github.com/edfloreshz/blueprint";
const APP_ICON: &[u8] = include_bytes!("../res/icons/hicolor/scalable/apps/icon.svg");

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    config_handler: Option<cosmic_config::Config>,
    // Configuration data that persists between application runs.
    config: Config,
    package: Option<Uuid>,
    shells: page::PageView,
    editors: page::PageView,
    languages: page::PageView,
    libraries: page::PageView,
    tools: page::PageView,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    OpenRepositoryUrl,
    SubscriptionChannel,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    NewPackage,
    EditPackage(Uuid),
    Page(Page, page::Message),
    PackageTitle(usize, String),
    TogglePackage(usize, bool),
    PackageDescription(usize, String),
}

/// Create a COSMIC application from the app model
impl Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "dev.edfloreshz.Blueprint";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // Create a nav bar with three page items.
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .text(fl!("shells"))
            .data::<Page>(Page::Shells)
            .icon(icon::from_name("utilities-terminal-symbolic"))
            .activate();

        nav.insert()
            .text(fl!("languages"))
            .data::<Page>(Page::Languages)
            .icon(icon::from_name("preferences-region-and-language-symbolic"));

        nav.insert()
            .text(fl!("editors"))
            .data::<Page>(Page::Editors)
            .icon(icon::from_name("accessories-text-editor-symbolic"));

        nav.insert()
            .text(fl!("libraries"))
            .data::<Page>(Page::Libraries)
            .icon(icon::from_name("address-book-new-symbolic"));

        nav.insert()
            .text(fl!("tools"))
            .data::<Page>(Page::Tools)
            .icon(icon::from_name("applications-utilities-symbolic"));

        let config = cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .map(|context| match Config::get_entry(&context) {
                Ok(config) => config,
                Err((errors, config)) => {
                    for error in errors {
                        log::error!("error loading app config: {}", error);
                    }

                    config
                }
            })
            .unwrap_or_default();

        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            nav,
            key_binds: HashMap::new(),
            // Optional configuration file for an application.
            config_handler: cosmic_config::Config::new(Self::APP_ID, Config::VERSION).ok(),
            config: config.clone(),
            package: None,
            shells: PageView::new(Page::Shells, config.clone()),
            languages: PageView::new(Page::Languages, config.clone()),
            editors: PageView::new(Page::Editors, config.clone()),
            libraries: PageView::new(Page::Libraries, config.clone()),
            tools: PageView::new(Page::Tools, config),
        };

        // Create a startup command that sets the window title.
        let command = app.update_title();

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![
            menu::Tree::with_children(
                menu::root(fl!("file")),
                menu::items(
                    &self.key_binds,
                    vec![menu::Item::Button(
                        fl!("new-package"),
                        MenuAction::NewPackage,
                    )],
                ),
            ),
            menu::Tree::with_children(
                menu::root(fl!("view")),
                menu::items(
                    &self.key_binds,
                    vec![menu::Item::Button(fl!("about"), MenuAction::About)],
                ),
            ),
        ]);

        vec![menu_bar.into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
            ContextPage::NewPackage | ContextPage::EditPackage => self.package_view(),
        })
    }

    fn on_context_drawer(&mut self) -> Command<Self::Message> {
        if !self.core.window.show_context {
            self.package = None;
        }
        Command::none()
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<Self::Message> {
        match self.nav.active_data::<Page>().cloned().unwrap_or_default() {
            Page::Shells => self.shells.view().map(|m| Message::Page(Page::Shells, m)),
            Page::Languages => self
                .languages
                .view()
                .map(|m| Message::Page(Page::Languages, m)),
            Page::Editors => self.editors.view().map(|m| Message::Page(Page::Editors, m)),
            Page::Libraries => self
                .libraries
                .view()
                .map(|m| Message::Page(Page::Libraries, m)),
            Page::Tools => self.tools.view().map(|m| Message::Page(Page::Tools, m)),
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;

        Subscription::batch(vec![
            // Create a subscription which emits updates through a channel.
            cosmic::iced::subscription::channel(
                std::any::TypeId::of::<MySubscription>(),
                4,
                move |mut channel| async move {
                    _ = channel.send(Message::SubscriptionChannel).await;

                    futures_util::future::pending().await
                },
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Commands may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let page = self.nav.active_data::<Page>().cloned().unwrap_or_default();
        let mut commands = vec![];

        match message {
            Message::OpenRepositoryUrl => {
                _ = open::that_detached(REPOSITORY);
            }
            Message::SubscriptionChannel => {
                // For example purposes only.
            }
            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                    self.package = None;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }

                // Set the title of the context drawer.
                self.set_context_title(context_page.title());
            }
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::EditPackage(id) => {
                self.package = Some(id);
                return Command::batch(vec![
                    self.update(Message::ToggleContextPage(ContextPage::EditPackage))
                ]);
            }
            Message::NewPackage => {
                let package = Package::new("Fish", Source::Apt("fish".into()), Page::Shells);
                let mut packages = self.config.packages.clone();
                packages.push(package);
                if let Some(config) = &mut self.config_handler {
                    if let Err(err) = self.config.set_packages(config, packages.clone()) {
                        log::error!("failed to set packages: {}", err);
                    }
                }

                return Command::batch(vec![
                    self.update(Message::UpdateConfig(self.config.clone())),
                    self.update(Message::Page(page, page::Message::ReloadPackages)),
                ]);
            }
            Message::Page(page, message) => {
                let page_commands = match page {
                    Page::Shells => self.shells.update(message),
                    Page::Languages => self.languages.update(message),
                    Page::Editors => self.editors.update(message),
                    Page::Libraries => self.libraries.update(message),
                    Page::Tools => self.tools.update(message),
                };
                for command in page_commands {
                    match command {
                        page::Command::EditPackage(id) => {
                            commands.push(self.update(Message::EditPackage(id)))
                        }
                    }
                }
            }
            Message::PackageTitle(index, title) => {
                let mut packages = self.config.packages.clone();
                packages[index].name = title;
                if let Some(config) = &mut self.config_handler {
                    if let Err(err) = self.config.set_packages(config, packages.clone()) {
                        log::error!("failed to set packages: {}", err);
                    }
                    commands.push(self.update(Message::Page(page, page::Message::ReloadPackages)));
                }
            }
            Message::PackageDescription(index, description) => {
                let mut packages = self.config.packages.clone();
                packages[index].description = description;
                if let Some(config) = &mut self.config_handler {
                    if let Err(err) = self.config.set_packages(config, packages.clone()) {
                        log::error!("failed to set packages: {}", err);
                    }
                    commands.push(self.update(Message::Page(page, page::Message::ReloadPackages)));
                }
            }
            Message::TogglePackage(index, toggled) => {
                let mut packages = self.config.packages.clone();
                packages[index].enabled = toggled;
                if let Some(config) = &mut self.config_handler {
                    if let Err(err) = self.config.set_packages(config, packages.clone()) {
                        log::error!("failed to set packages: {}", err);
                    }
                    commands.push(self.update(Message::Page(page, page::Message::ReloadPackages)));
                }
            }
        }

        Command::batch(commands)
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);

        let page = self.nav.data::<Page>(id).cloned().unwrap_or_default();

        Command::batch(vec![
            self.update_title(),
            self.update(Message::Page(page, page::Message::ReloadPackages)),
        ])
    }
}

impl AppModel {
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));

        let title = widget::text::title3(fl!("app-title"));

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::OpenRepositoryUrl)
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    pub fn package_view(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let Some(id) = self.package else {
            let create = widget::button(widget::text(fl!("create")))
                .on_press(Message::NewPackage)
                .padding(0);

            return widget::column()
                .push(create)
                .align_items(Alignment::Center)
                .spacing(space_xxs)
                .into();
        };

        let Some(index) = self.config.packages.iter().position(|p| p.id == id) else {
            return widget::text("No package selected").into();
        };

        let package = self.config.packages[index].clone();

        let title =
            widget::settings::item_row(vec![widget::text_input(fl!("name"), package.name.clone())
                .label("Name")
                .on_input(move |text| Message::PackageTitle(index, text))
                .into()]);

        let description = widget::settings::item_row(vec![widget::text_input(
            fl!("description"),
            package.description.clone(),
        )
        .label("Description")
        .on_input(move |text| Message::PackageDescription(index, text))
        .into()]);

        let enabled = widget::settings::item(
            fl!("enabled"),
            widget::checkbox(fl!("enabled"), package.enabled, move |toggled| {
                Message::TogglePackage(index, toggled)
            }),
        );

        widget::settings::view_section(fl!("package"))
            .add(title)
            .add(description)
            .add(enabled)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Command<Message> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        self.set_window_title(window_title)
    }
}

/// The page to display in the application.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Page {
    #[default]
    Shells,
    Languages,
    Editors,
    Libraries,
    Tools,
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Self::Shells => fl!("shells"),
            Self::Languages => fl!("languages"),
            Self::Editors => fl!("editors"),
            Self::Libraries => fl!("libraries"),
            Self::Tools => fl!("tools"),
        }
    }
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    NewPackage,
    EditPackage,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::NewPackage => fl!("new-package"),
            Self::EditPackage => fl!("edit-package"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    NewPackage,
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::NewPackage => Message::ToggleContextPage(ContextPage::NewPackage),
        }
    }
}
