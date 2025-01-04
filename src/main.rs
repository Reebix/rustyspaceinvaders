mod drawings;

use crate::drawings::{get_bullet, get_player};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

//00000000 00000000 00000000
//r        g        b
fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn insert_drawing(buffer: &mut [u32], x: usize, y: usize, drawing: &drawings::Drawing) {
    for i in 0..drawing.height {
        for j in 0..drawing.width {
            let index = (y + i) * WIDTH + x + j;
            if index < buffer.len() {
                buffer[index] = drawing.pixels[i * drawing.width + j];
            }
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let player_speed = 4;
    let bullet_speed = 8;

    let mut bullets = vec![];

    let mut player_pos = (WIDTH / 2, HEIGHT - 50);

    // println!("{}", from_rgb(0, 255, 0));

    let mut window = Window::new(
        "Rusty Space Invaders",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // clear the buffer
        for i in buffer.iter_mut() {
            *i = 0;
        }

        // update bullets
        let mut new_bullets = vec![];
        bullets.iter_mut().for_each(|bullet: &mut (usize, usize)| {
            let new_bullet = (bullet.0 as isize, bullet.1 as isize - bullet_speed);
            if new_bullet.1 > 0 {
                new_bullets.push((new_bullet.0 as usize, new_bullet.1 as usize));
            }
        });
        bullets = new_bullets;

        // draw bullets
        bullets.iter_mut().for_each(|bullet: &mut (_, _)| {
            insert_drawing(
                &mut buffer,
                bullet.0 + get_player().width / 2,
                bullet.1,
                &get_bullet(),
            );
        });

        insert_drawing(&mut buffer, player_pos.0, player_pos.1, &get_player());

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            bullets.push((player_pos.0, player_pos.1));
        }

        if window.is_key_down(Key::Left) && player_pos.0 > 0 {
            player_pos.0 -= player_speed;
        }

        if window.is_key_down(Key::Right) && player_pos.0 < WIDTH - get_player().width {
            player_pos.0 += player_speed;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
