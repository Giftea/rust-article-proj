use crossterm::style::{ Color::Rgb, Color::Yellow};
use termimad::{MadSkin, StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_bg(Rgb {
        r: 28,
        g: 28,
        b: 28,
    });
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.set_headers_fg(Yellow);
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '▐');
    skin.quote_mark.set_fg(Rgb {
        r: 242,
        g: 184,
        b: 113,
    });
    skin.inline_code.set_fg(Rgb {
        r: 238,
        g: 133,
        b: 238,
    });
    skin.italic.set_fg(Rgb {
        r: 215,
        g: 255,
        b: 0,
    });

    skin
}