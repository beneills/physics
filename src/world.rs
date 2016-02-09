    pub struct World {
        pub x_max: u32,
        pub y_max: u32
    }

    impl World {
        pub fn new(x_max: u32, y_max: u32) -> World {
            World {
                x_max: x_max,
                y_max: y_max
            }
        }
    }
