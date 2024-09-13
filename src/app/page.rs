// SPDX-License-Identifier: GPL-3.0

use cosmic::{app::Command, widget, Apply, Element};

use super::models::package::Package;

pub struct PageView {
    title: String,
    packages: Vec<Package>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPackages(Vec<Package>),
}

impl PageView {
    pub fn new(title: String, packages: Vec<Package>) -> Self {
        Self { title, packages }
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
            Message::SetPackages(packages) => self.packages = packages,
        }
        Command::none()
    }
}
