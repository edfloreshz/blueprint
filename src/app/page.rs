// SPDX-License-Identifier: GPL-3.0

use cosmic::{
    cosmic_config::{self, CosmicConfigEntry},
    iced::Length,
    prelude::CollectionWidget,
    widget::{self, icon},
    Application, Apply, Element,
};
use uuid::Uuid;

use crate::{config::Config, fl};

use super::{models::package::Package, AppModel, Page};

pub struct PageView {
    page: Page,
    config: Config,
    config_handler: Option<cosmic_config::Config>,
    title: String,
    packages: Vec<Package>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ReloadPackages,
    EditPackage(Uuid),
}

#[derive(Debug, Clone)]
pub enum Command {
    EditPackage(Uuid),
}

impl PageView {
    pub fn new(page: Page, config: Config) -> Self {
        let title = page.to_string();
        let packages = config
            .packages
            .iter()
            .cloned()
            .filter(|p| p.page == page)
            .collect();

        Self {
            page,
            config,
            config_handler: cosmic_config::Config::new(AppModel::APP_ID, Config::VERSION).ok(),
            title,
            packages,
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let mut section = widget::settings::view_section(fl!("packages"));
        let packages: Vec<Element<'a, Message>> = self
            .packages
            .iter()
            .map(|package| Self::package_row(package))
            .collect();

        if packages.is_empty() {
            section = section.add(widget::text(fl!("no-packages")));
        }

        for package in packages {
            section = section.add(package);
        }

        widget::column()
            .push(widget::text::title2(self.title.clone()))
            .push(section)
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }

    pub fn package_row<'a>(package: &Package) -> Element<'a, Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        let description = if package.description.is_empty() {
            None
        } else {
            Some(widget::text::caption(package.description.clone()))
        };

        widget::settings::item_row(vec![
            widget::column()
                .push(widget::text(package.name.clone()))
                .push_maybe(description)
                .spacing(spacing.space_xxxs)
                .into(),
            widget::horizontal_space(Length::Fill).into(),
            widget::button(icon::from_name("view-more-symbolic"))
                .on_press(Message::EditPackage(package.id.clone()))
                .into(),
        ])
        .into()
    }

    pub fn update(&mut self, message: Message) -> Vec<Command> {
        let mut commands = vec![];
        match message {
            Message::ReloadPackages => {
                if let Some(context) = &self.config_handler {
                    if let Ok(config) = Config::get_entry(context) {
                        self.config = config;
                    }
                }
                let packages = self
                    .config
                    .packages
                    .iter()
                    .cloned()
                    .filter(|p| p.page == self.page)
                    .collect();

                self.packages = packages
            }
            Message::EditPackage(id) => commands.push(Command::EditPackage(id)),
        }
        commands
    }
}
