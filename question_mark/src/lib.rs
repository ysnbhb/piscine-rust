pub struct Four {
    pub fourth_layer: Option<u16>,
}

pub struct Three {
    pub third_layer: Option<Four>,
}

pub struct Two {
    pub second_layer: Option<Three>,
}

pub struct One {
    pub first_layer: Option<Two>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer
            .unwrap()
            .second_layer
            .unwrap()
            .third_layer
            .unwrap()
            .fourth_layer
    }
}
