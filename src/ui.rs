use raylib::prelude::*;

use crate::{board, capture, coord, piece};

pub const WINDOW_SIZE: u16 = 640;
pub const TILE_SIZE: u16 = WINDOW_SIZE / 8;

pub fn draw_capture_hints(
    d: &mut RaylibDrawHandle,
    vec: &Vec<Box<capture::Capture>>,
    level: u8,
    max_depth: u8,
    reached: Option<&mut bool>,
) -> bool {
    if let Some(done) = reached {
        for i in vec {
            if i.next.len() > 0 {
                draw_capture_hints(d, &i.next, level + 1, max_depth, Some(done));
            }
            if (level == max_depth) || *done {
                let (mut x, mut y) = coord::xy_from_n(i.ndest);
                d.draw_circle(
                    (x as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    (y as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    ((TILE_SIZE - 10) / 2) as f32,
                    color::rcolor(0xFF, 0xFF, 0, 0x5F),
                );
                (x, y) = coord::xy_from_n(i.ncapture);
                d.draw_rectangle(
                    (x as u16 * TILE_SIZE) as i32,
                    (y as u16 * TILE_SIZE) as i32,
                    TILE_SIZE as i32,
                    TILE_SIZE as i32,
                    color::rcolor(0xFF, 0x6F, 0, 0x5F),
                );
                *done = true;
            }
            if level == 0 {
                *done = false;
            }
        }
    } else {
        let mut done: bool = false;
        draw_capture_hints(d, vec, level, max_depth, Some(&mut done));
    }
    return vec.len() > 0;
}

pub fn draw_hints(
    d: &mut RaylibDrawHandle,
    piece: Option<&mut piece::Piece>,
    tiles: &Vec<u8>,
    capture_available: bool,
) {
    if !capture_available {
        if let Some(piece2) = piece {
            let vec: Vec<u8> = piece::possible_moves(piece2, tiles);
            for i in vec {
                let (x, y) = coord::xy_from_n(i);
                d.draw_circle(
                    (x as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    (y as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    ((TILE_SIZE - 10) / 2) as f32,
                    color::rcolor(0xFF, 0xFF, 0, 0x5F),
                );
            }
        }
    }
}

pub fn draw_board(d: &mut RaylibDrawHandle) {
    for row in 0..8 {
        for col in 0..8 {
            let x = col * TILE_SIZE;
            let y = row * TILE_SIZE;
            let color = if (row + col) % 2 == 0 {
                raylib::color::rcolor(0x7B, 0x7B, 0x7B, 0xFF)
            } else {
                raylib::color::rcolor(0xDF, 0xDF, 0xDF, 0xFF)
            };
            d.draw_rectangle(
                x as i32,
                y as i32,
                TILE_SIZE as i32,
                TILE_SIZE as i32,
                color,
            );
        }
    }
}

pub fn draw_pieces(d: &mut RaylibDrawHandle, board: &mut board::Board) {
    for i in 0u8..32u8 {
        if board.tiles[i as usize] > 0
            && board.tiles[i as usize] < 32
            && board.pieces[(board.tiles[i as usize] - 1) as usize].valid
        {
            let (x, y) = coord::xy_from_n(i + 1);
            let piece: &piece::Piece =
                piece::from_n(i + 1, board).expect("error: no valid piece found");
            d.draw_circle(
                (x as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                (y as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                ((TILE_SIZE - 10) / 2) as f32,
                if piece.player {
                    Color::WHITE
                } else {
                    Color::BLACK
                },
            );
            if piece.king {
                d.draw_circle(
                    (x as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    (y as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    ((TILE_SIZE - 30) / 2) as f32,
                    color::rcolor(0x7B, 0x7B, 0x7B, 255),
                );
                d.draw_circle(
                    (x as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    (y as u16 * TILE_SIZE + TILE_SIZE / 2) as i32,
                    ((TILE_SIZE - 50) / 2) as f32,
                    if piece.player {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    },
                );
            }
        }
    }
}
