use unicode_width::UnicodeWidthStr;

use zellij_tile::prelude::*;
use zellij_tile_utils::style;

use crate::{
    color::ModeColor,
    view::{Block, View},
};

pub struct Tabs;

impl Tabs {
    pub fn render(tabs: &[TabInfo], mode: InputMode, palette: Palette) -> View {
        let mut blocks: Vec<Block> = Vec::with_capacity(tabs.len());
        let mut total_len = 0;

        for tab in tabs {
            let block = Tab::render(tab, mode, palette);

            total_len += block.len;
            blocks.push(block);
        }

        View {
            blocks,
            len: total_len,
        }
    }
}

pub struct Tab;

impl Tab {
    pub fn render(tab: &TabInfo, mode: InputMode, palette: Palette) -> Block {
        let mut text = tab.name.clone();

        if tab.active && mode == InputMode::RenameTab && text.is_empty() {
            text = String::from("Enter name...");
        }

        if tab.is_sync_panes_active {
            text.push_str(" [sync]");
        }

        if text.len() < 16 {
            text = format!("{:^16}", text);
        } else {
            text = format!(" {} ", text);
        }

        let len = text.width();

        let ModeColor { mode_color } = ModeColor::new(mode, palette);

        let fg = if tab.active {
            mode_color
        } else {
            palette.fg
        };

        let bg = if tab.active {
            palette.black
        } else {
            palette.bg
        };

        let body = style!(fg, bg).bold().paint(text);

        Block {
            body: body.to_string(),
            len,
            tab_index: Some(tab.position),
        }
    }
}
