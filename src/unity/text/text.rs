use bevy::{color::palettes::css::BLACK, prelude::*};

pub struct TextOut<'a> {
    text: &'a str,
    text_font: TextFont,
    text_shadow: TextShadow,
    text_color: TextColor,
}

impl<'a> TextOut<'a> {
    pub fn init(text: &'a str) -> Self {
        Self {
            text: text,
            text_font: TextFont {
                font_size: 22.,
                ..default()
            },
            text_shadow: TextShadow {
                offset: Vec2 { x: 3., y: 3. },
                color: BLACK.with_alpha(0.5).into(),
            },
            text_color: TextColor::BLACK,
        }
    }

    pub fn out(self) -> impl Bundle {
        (
            Text::new(self.text),
            self.text_font,
            self.text_shadow,
            self.text_color,
        )
    }

    pub fn size(mut self, size: f32) -> Self {
        self.text_font = self.text_font.with_font_size(size);
        self
    }

    pub fn set_color(mut self, color: Srgba) -> Self {
        self.text_color = color.into();
        self
    }

    pub fn set_shadow(mut self, shadow: TextShadow) -> Self {
        self.text_shadow = shadow;
        self
    }

    pub fn set_shadow_size(mut self, size: Vec2) -> Self {
        self.text_shadow = TextShadow {
            offset: size,
            color: self.text_shadow.color,
        };
        self
    }

    pub fn set_no_shadow(mut self) -> Self {
        self.text_shadow = TextShadow {
            offset: Vec2 { x: 0., y: 0. },
            color: self.text_shadow.color,
        };
        self
    }

    pub fn set_shadow_color(mut self, color: Srgba) -> Self {
        self.text_shadow = TextShadow {
            offset: self.text_shadow.offset,
            color: color.into(),
        };
        self
    }
}
