// Define each struct with public fields

pub struct One {
    pub first_layer: Option<Two>,
}

pub struct Two {
    pub second_layer: Option<Three>,
}

pub struct Three {
    pub third_layer: Option<Four>,
}

pub struct Four {
    pub fourth_layer: Option<u16>,
}

// Implement the function using the `?` operator
impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        let two = self.first_layer?;           // Option<Two> → Two
        let three = two.second_layer?;         // Option<Three> → Three
        let four = three.third_layer?;         // Option<Four> → Four
        let value = four.fourth_layer?;        // Option<u16> → u16
        Some(value)
    }
}
