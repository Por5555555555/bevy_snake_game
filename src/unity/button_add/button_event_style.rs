use bevy::{color::palettes::css::*, prelude::*};

pub struct ButtonStyle {
    def_bg_color: BackgroundColor,
    hov_bg_color: BackgroundColor,
    //border_color: BorderColor,
}

impl ButtonStyle {
    pub fn init() -> Self {
        Self {
            def_bg_color: BackgroundColor(DARK_GRAY.into()),
            hov_bg_color: BackgroundColor(DARK_CYAN.into()),
            //border_color: BorderColor::all(BLACK),
        }
    }

    pub fn add_event(self, inter: Interaction, mut color: Mut<'_, BackgroundColor>) {
        match inter {
            Interaction::Hovered => {
                *color = self.hov_bg_color;
            }
            Interaction::None => {
                *color = self.def_bg_color;
            }
            _ => {}
        }
    }
}
