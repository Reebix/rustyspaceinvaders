// hide console
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod drawings;

use crate::drawings::{get_bullet, get_invader, get_number, get_player};
use minifb::{Key, Window, WindowOptions};
use rand::random;

const WIDTH: usize = 480;
const HEIGHT: usize = 450;

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
    let mut invader_speed: isize = 1;
    let time_to_shoot = 60;
    let mut time = 0;
    let mut lives = 3;
    let mut iteration = 1;
    let mut score = 0;

    let mut invader_pos: (usize, usize) = (30, 30);
    let mut player_pos = (WIDTH / 2, HEIGHT - 50);

    let mut bullets = vec![];
    let mut invaders = [true; 12 * 4];
    let mut invader_bullets = vec![];

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

    let mut finished = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if lives == 0 {
            if !finished {
                buffer = vec![0; WIDTH * HEIGHT];
                for (i, c) in score.to_string().chars().enumerate() {
                    insert_drawing(
                        &mut buffer,
                        i * get_number('0').width + WIDTH / 2
                            - get_number('0').width * score.to_string().len() / 2,
                        WIDTH / 2,
                        &get_number(c),
                    );
                }
                finished = true;
            }
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            continue;
        }

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

        // draw invader bullets
        invader_bullets
            .iter_mut()
            .for_each(|bullet: &mut (isize, isize)| {
                insert_drawing(
                    &mut buffer,
                    bullet.0 as usize,
                    bullet.1 as usize,
                    &get_bullet(),
                );
            });

        // update invader bullets
        let mut new_invader_bullets = vec![];
        invader_bullets
            .iter_mut()
            .for_each(|bullet: &mut (isize, isize)| {
                let new_bullet = (bullet.0, bullet.1 + bullet_speed);
                if new_bullet.1 < HEIGHT as isize {
                    new_invader_bullets.push(new_bullet);
                }
            });
        invader_bullets = new_invader_bullets;

        // draw player
        insert_drawing(&mut buffer, player_pos.0, player_pos.1, &get_player());

        // update invaders
        invader_pos.0 = (invader_pos.0 as isize + invader_speed).max(10) as usize;
        if invader_pos.0 > (WIDTH - 12 * 30) || invader_pos.0 == 10 {
            invader_speed = -invader_speed;
            invader_pos.1 += 10;
        }

        if time % (time_to_shoot / iteration) == 0 {
            // randomly select with rand::random
            let index = random::<u8>() % 12;

            // get highest invader in the column
            for j in (0..4).rev() {
                if invaders[index as usize * 4 + j] {
                    invader_bullets.push((
                        (invader_pos.0 + index as usize * 30) as isize,
                        (invader_pos.1 + j * 30) as isize,
                    ));
                    break;
                }
            }
        }

        // draw invaders
        for i in 0..12 {
            for j in 0..4 {
                if invaders[i * 4 + j] {
                    insert_drawing(
                        &mut buffer,
                        (invader_pos.0 + (i * 30)),
                        invader_pos.1 + j * 30,
                        &get_invader(),
                    );
                }
            }
        }

        // check for collisions
        let mut remaining_bullets = vec![];
        for bullet in bullets.iter() {
            let mut hit = false;
            for i in 0..12 {
                for j in 0..4 {
                    let invader_x = invader_pos.0 + (i * 30);
                    let invader_y = invader_pos.1 + j * 30;
                    if invaders[i * 4 + j]
                        && bullet.0 >= invader_x
                        && bullet.0 <= invader_x + 24
                        && bullet.1 >= invader_y
                        && bullet.1 <= invader_y + 24
                    {
                        invaders[i * 4 + j] = false;
                        hit = true;
                        break;
                    }
                }
                if hit {
                    score += iteration * 10;
                    break;
                }
            }
            if !hit {
                remaining_bullets.push(*bullet);
            }
        }
        bullets = remaining_bullets;

        // draw lives
        for i in 0..lives {
            insert_drawing(&mut buffer, WIDTH - (40 + i * 30), 10, &get_player());
        }

        // draw score
        for (i, c) in score.to_string().chars().enumerate() {
            insert_drawing(
                &mut buffer,
                10 + i * get_number('0').width,
                10,
                &get_number(c),
            );
        }

        // check for collisions with player
        let mut remaining_invader_bullets = vec![];
        for bullet in invader_bullets.iter() {
            let mut hit = false;
            if bullet.0 >= player_pos.0 as isize
                && bullet.0 <= player_pos.0 as isize + 24
                && bullet.1 >= player_pos.1 as isize
                && bullet.1 <= player_pos.1 as isize + 24
            {
                hit = true;
                lives -= 1;
            }
            if !hit {
                remaining_invader_bullets.push(*bullet);
            }
        }
        invader_bullets = remaining_invader_bullets;

        // when all invaders are gone, reset the game
        if invaders.iter().all(|&x| !x) {
            invaders = [true; 12 * 4];
            invader_pos = (30, 30);
            bullets = vec![];
            iteration += 1;
        }

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
            || window.is_key_pressed(Key::W, minifb::KeyRepeat::No)
        {
            bullets.push((player_pos.0, player_pos.1));
        }
        if (window.is_key_down(Key::Left) || window.is_key_down(Key::A)) && player_pos.0 > 0 {
            player_pos.0 -= player_speed;
        }

        if (window.is_key_down(Key::Right) || window.is_key_down(Key::D))
            && player_pos.0 < WIDTH - get_player().width
        {
            player_pos.0 += player_speed;
        }

        time += 1;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
