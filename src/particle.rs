    use world;

//     macro_rules! new_position {
//     ( $current_position:expr, $velocity:expr ) => {
//         {
//             let pos = (i32) current_position: i32;
//             pos += velocity;
//             if ( pos < 0 ) {
//                 pos = 0;
//             }
//             (u32) pos
//         }
//     };
// }

    pub struct Particle<'a> {
        world: &'a world::World,
        x: i32,
        y: i32,
        radius: u32,
        x_velocity: i32,
        y_velocity: i32
    }

    impl <'a>Particle<'a> {
        pub fn new(world: &world::World,
                   x_initial: i32,
                   y_initial: i32,
                   radius_initial: u32,
                   x_velocity_initial: i32,
                   y_velocity_initial: i32) -> Particle {
            Particle {
                world: world,
                x: x_initial,
                y: y_initial,
                radius: radius_initial,
                x_velocity: x_velocity_initial,
                y_velocity: y_velocity_initial
            }
        }

        pub fn touching_edge_of_world_horizontal(&self) -> bool {
            return (self.x - (self.radius as i32) <= 0               ) ||
                   (self.x + (self.radius as i32) >= (self.world.x_max as i32));
        }

        pub fn touching_edge_of_world_vertical(&self) -> bool {
            return (self.y - (self.radius as i32) <= 0               ) ||
                   (self.y + (self.radius as i32) >= (self.world.y_max as i32));
        }

        pub fn touching_edge_of_world(&self) -> bool {
            return self.touching_edge_of_world_horizontal() ||
                   self.touching_edge_of_world_vertical();
        }

        pub fn bounce_horizontal(&mut self) {
            self.x_velocity = -self.x_velocity;
        }

        pub fn bounce_vertical(&mut self) {
            self.y_velocity = -self.y_velocity;
        }

        pub fn step(&mut self) {
            self.x += self.x_velocity;
            self.y += self.y_velocity;
        }

        pub fn bounding_rectangle(&self) -> [f64; 4] {
            return [(self.x - (self.radius as i32)) as f64,
                    (self.y - (self.radius as i32)) as f64,
                    (2 * self.radius) as f64,
                    (2 * self.radius) as f64
                    ];
        }
    }
