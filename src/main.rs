extern crate piston_window;
// extern crate gfx;
// extern crate gfx_device_gl;

mod world;
mod particle;
mod graphics;

use particle::Particle;

fn main() {
    let w = world::World::new(640, 480);
    let p1 = particle::Particle::new(&w, 10, 20, 5, 2, 3);
    let p2 = particle::Particle::new(&w, 40, 190, 10, 8, 2);
    let p3 = particle::Particle::new(&w, 110, 190, 100, -4, -3);

    println!("p1 is touching wall: {}", p1.touching_edge_of_world());
    println!("p2 is touching wall: {}", p2.touching_edge_of_world());

    let mut particles = vec![p1, p2, p3];

    let mut g = graphics::Graphics::new();
    g.draw_window();
    g.game_loop(&mut particles);
}
