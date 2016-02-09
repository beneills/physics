use piston_window::{ WindowSettings, PistonWindow, ellipse, rectangle, clear };

use particle::Particle;

pub struct Graphics {
    window: Option<PistonWindow>
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics {
            window: None
        }
    }

    pub fn draw_window(&mut self) {
        self.window = Some(WindowSettings::new("Physics - physics engine", [640, 480]).exit_on_esc(true).into())
    }

    pub fn game_loop(self, particles: &mut Vec<Particle>) {
        for e in self.window.unwrap() {

            e.draw_2d(|c, g| {
                for p in particles.iter_mut() {
                     p.step();
                     if p.touching_edge_of_world_horizontal() {
                         p.bounce_horizontal();
                     }

                     if p.touching_edge_of_world_vertical() {
                         p.bounce_vertical();
                     }
                 }
                clear([1.0; 4], g);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                    [0.0, 0.0, 100.0, 100.0],
                    c.transform, g);
                ellipse([1.0, 0.0, 0.0, 1.0],
                    [150.0, 150.0, 200.0, 200.0],
                    c.transform, g);

                for p in particles.iter_mut() {
                    ellipse([0.0, 0.0, 0.0, 1.0],
                            p.bounding_rectangle(),
                            c.transform, g);
                }
                println!("draw_2d");
            });
        }
    }
}
