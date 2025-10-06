use bevy::prelude::*;
use getset::Setters;

use crate::unity::node_add::addon_node::AddOnNode;

#[derive(Setters)]
pub struct ButtonAddOn {
    back_ground_color: BackgroundColor,
    border_color: BorderColor,
    border_radius: BorderRadius,
    node: AddOnNode,
}

impl ButtonAddOn {
    pub fn init() -> Self {
        Self {
            back_ground_color: BackgroundColor(Color::WHITE),
            border_radius: BorderRadius::all(px(3)),
            border_color: BorderColor::all(Color::BLACK),
            node: AddOnNode::init(),
        }
    }

    pub fn border_radium(mut self, size: i32) -> Self {
        self.border_radius = BorderRadius::all(px(size));
        self
    }

    pub fn node(mut self, node: AddOnNode) -> Self {
        self.node = node;
        self
    }

    pub fn set_bg_color(mut self, color: Srgba) -> Self {
        self.back_ground_color = BackgroundColor(color.into());
        self
    }

    pub fn set_border_color(mut self, color: Srgba) -> Self {
        self.border_color = BorderColor::all(color);
        self
    }

    pub fn out(self) -> impl Bundle {
        info!("{:#?}", self.back_ground_color);
        (
            Button,
            self.border_radius,
            self.border_color,
            self.back_ground_color,
            self.node.get(),
        )
    }
}
