use bevy::prelude::*;

pub struct AddOnNode {
    node: Node,
}

impl AddOnNode {
    pub fn init() -> Self {
        Self {
            node: Node { ..default() },
        }
    }

    pub fn get(self) -> Node {
        self.node
    }

    pub fn mode_main(mut self) -> Self {
        self = AddOnNode::init();
        self.node.width = percent(100);
        self.node.height = percent(100);
        self.node.justify_content = JustifyContent::Center;
        self.node.align_items = AlignItems::Center;
        self
    }

    pub fn mode_head(mut self) -> Self {
        self = AddOnNode::init();
        self.node.width = px(500);
        self.node.height = px(500);
        self.node.justify_content = JustifyContent::Center;
        self.node.flex_direction = FlexDirection::Column;
        self.node.align_items = AlignItems::Center;
        self
    }

    pub fn set_width(mut self, new_width: i32) -> Self {
        self.node.width = px(new_width);
        self
    }

    pub fn set_height(mut self, new_height: i32) -> Self {
        self.node.height = px(new_height);
        self
    }

    pub fn align_new(mut self, align_items: AlignItems) -> Self {
        self.node.align_items = align_items;
        self
    }

    pub fn mode_button(mut self) -> Self {
        self.node.width = px(165);
        self.node.height = px(50);
        self.node.justify_content = JustifyContent::Center;
        self.node.align_items = AlignItems::Center;
        self.node.margin = UiRect::all(px(10));
        self.node.border = UiRect::all(px(3));
        self
    }
}
