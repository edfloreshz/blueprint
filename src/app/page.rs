// SPDX-License-Identifier: GPL-3.0

use cosmic::{widget, Apply, Element};

#[derive(Debug, Clone)]
pub enum Message {}

pub fn view<'a>(title: String) -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2(title))
        .apply(widget::container)
        .center_y()
        .center_x()
        .into()
}
