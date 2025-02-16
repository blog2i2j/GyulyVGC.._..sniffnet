#![allow(clippy::unreadable_literal)]

//! Themes optimized for OLED displays and visually impaired users
//! <https://github.com/GyulyVGC/sniffnet/pull/708>

use iced::color;
use once_cell::sync::Lazy;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

pub static OLED_DARK_PALETTE: Lazy<Palette> = Lazy::new(|| Palette {
    primary: color!(0x000000),
    secondary: color!(0x934900),
    outgoing: color!(0xF0F0F0),
    starred: color!(0xFFFF00),
    text_headers: color!(0xE0E0E0),
    text_body: color!(0xfcfaf0),
});

pub static OLED_DARK_PALETTE_EXTENSION: Lazy<PaletteExtension> =
    Lazy::new(|| OLED_DARK_PALETTE.generate_palette_extension());

pub static OLED_LIGHT_PALETTE: Lazy<Palette> = Lazy::new(|| Palette {
    primary: color!(0xFFFFFF),
    secondary: color!(0x6CB6FF),
    outgoing: color!(0x0F0F0F),
    starred: color!(0x0000FF),
    text_headers: color!(0x1F1F1F),
    text_body: color!(0x03050F),
});

pub static OLED_LIGHT_PALETTE_EXTENSION: Lazy<PaletteExtension> =
    Lazy::new(|| OLED_LIGHT_PALETTE.generate_palette_extension());
