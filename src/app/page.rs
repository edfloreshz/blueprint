// SPDX-License-Identifier: GPL-3.0

use cosmic::{app::Command, widget, Apply, Element};

use crate::config::Config;

use super::{models::package::Package, Page};

pub struct PageView {
    page: Page,
    config: Config,
    title: String,
    packages: Vec<Package>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ReloadPackages,
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
            title,
            packages,
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let packages = self
            .packages
            .iter()
            .map(|package| Self::package_view(package))
            .collect();

        widget::column()
            .push(widget::text::title2(self.title.clone()))
            .push(widget::column::with_children(packages))
            .apply(widget::container)
            .center_y()
            .center_x()
            .into()
    }

    pub fn package_view<'a>(package: &Package) -> Element<'a, Message> {
        widget::row()
            .push(widget::text(package.name.clone()))
            .into()
    }

    pub fn update(&mut self, message: Message) -> Command<crate::app::Message> {
        match message {
            Message::ReloadPackages => {
                let packages = self
                    .config
                    .packages
                    .iter()
                    .cloned()
                    .filter(|p| p.page == self.page)
                    .collect();

                self.packages = packages
            }
        }
        Command::none()
    }
}
