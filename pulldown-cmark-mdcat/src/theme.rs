// Copyright 2018-2020 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Provide a colour theme for mdcat.

use anstyle::{AnsiColor, Color, Style};

/// A colour theme for mdcat.
///
/// Currently you cannot create custom styles, but only use the default theme via [`Theme::default`].
#[derive(Debug, Clone)]
pub struct Theme {
    /// Style for HTML blocks.
    pub html_block_style: Style,
    /// Style for inline HTML.
    pub inline_html_style: Style,
    /// Style for code, unless the code is syntax-highlighted.
    pub code_style: Style,
    /// Style for links.
    pub link_style: Style,
    /// Color for image links (unless the image is rendered inline)
    pub image_link_style: Style,
    /// Color for rulers.
    pub rule_color: Color,
    /// Color for borders around code blocks.
    pub code_block_border_color: Color,
    /// Color for headings
    pub heading_style: Style,
}

impl Default for Theme {
    /// The default theme from mdcat 1.x
    fn default() -> Self {
        Self {
            html_block_style: Style::new().fg_color(Some(AnsiColor::Green.into())),
            inline_html_style: Style::new().fg_color(Some(AnsiColor::Green.into())),
            code_style: Style::new().fg_color(Some(AnsiColor::Yellow.into())),
            link_style: Style::new().fg_color(Some(AnsiColor::Blue.into())),
            image_link_style: Style::new().fg_color(Some(AnsiColor::Magenta.into())),
            rule_color: AnsiColor::Green.into(),
            code_block_border_color: AnsiColor::Green.into(),
            heading_style: Style::new().fg_color(Some(AnsiColor::Blue.into())).bold(),
        }
    }
}

/// Combine styles.
pub trait CombineStyle {
    /// Put this style on top of the other style.
    ///
    /// Return a new style which falls back to the `other` style for all style attributes not
    /// specified in this style.
    fn on_top_of(self, other: &Self) -> Self;
}

impl CombineStyle for Style {
    /// Put this style on top of the `other` style.
    fn on_top_of(self, other: &Style) -> Style {
        Style::new()
            .fg_color(self.get_fg_color().or(other.get_fg_color()))
            .bg_color(self.get_bg_color().or(other.get_bg_color()))
            .effects(other.get_effects() | self.get_effects())
            .underline_color(self.get_underline_color().or(other.get_underline_color()))
    }
}
