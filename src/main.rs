use macroquad::prelude::*;
use std::thread;
//use std::time::{Duration, Instant};
use macroquad::time::get_fps;

mod common;
mod grid;
mod particule;
mod ui;
mod world;

use common::*;
use grid::*;
use particule::*;
use ui::*;
use world::*;

#[macroquad::main("Particules")]
async fn main() {
    let mouse_pointer = Particule {
        size: Some(Size {
            width: 4.0,
            height: 4.0,
        }),
        color: Some(RED),
        ptype: ParticuleType::POINTER,
    };

    const SIZE: f32 = 3.0;

    let mut world = World {
        particule_size: SIZE,
        grid: Grid::new(Size {
            width: SIZE,
            height: SIZE,
        }),
        settings: Settings {
            selected_particule_type: ParticuleType::SAND,
        },
    };

    let mut stats = Stats {
        created_particules_count: world.generate_particules(),
        world_particules: world.count_particules(),
        fps: 0,
    };

    loop {
        clear_background(BLACK);

        stats.world_particules = world.count_particules();
        handle_ui(&mut world.settings, &stats);

        let mut mouse_pointer_position = Position { x: 0.0, y: 0.0 };

        (mouse_pointer_position.x, mouse_pointer_position.y) = mouse_position();

        if is_mouse_button_released(MouseButton::Left) || is_key_down(KeyCode::LeftControl) {
            stats.created_particules_count += 1;
            let p = world.create_particule(world.settings.selected_particule_type.clone());

            // let position = world.fit_position(mouse_pointer_position.clone());
            world.put_particule(
                p,
                Grid::position_to_tile(
                    world.grid.tile_size.clone(),
                    mouse_pointer_position.clone(),
                ),
            );
        }

        world.update();
        world.draw();
        mouse_pointer.draw(mouse_pointer_position);

        //thread::sleep(Duration::from_millis(3));
        stats.fps = get_fps();
        next_frame().await;
    }
}
