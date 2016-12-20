/*
 * Some various functions for rendering at particular depths
 */

#![allow(dead_code)]

use image::Color;
use std::u8;

const RGB_SCALE_ITERS:       u32 = 1536;
const LIGHT_RGB_SCALE_ITERS: u32 = 768;

pub fn rgb_scale(depth: f64) -> Color {
    let descreet_depth = (depth * RGB_SCALE_ITERS as f64) as u32;
    match descreet_depth {
           0 ...  255 => Color::RGB(
            u8::MAX, 
            descreet_depth as u8, 
            0u8,
        ),
         256 ...  511 => Color::RGB(
            (511 - descreet_depth) as u8,
            u8::MAX,
            0u8,
        ),
         512 ...  767 => Color::RGB(
            0u8,
            u8::MAX,
            (descreet_depth - 512) as u8, 
        ),
         768 ... 1023 => Color::RGB(
            0u8,
            (1023 - descreet_depth) as u8,
            u8::MAX,
        ),
        1024 ... 1279 => Color::RGB(
            (descreet_depth - 1024) as u8, 
            0u8,
            u8::MAX,
        ),
        1280 ... 1535 => Color::RGB(
            u8::MAX,
            0u8,
            (1535 - descreet_depth) as u8,
        ),
        RGB_SCALE_ITERS => Color::Black,
        _ => Color::Black,
    }
}

pub fn light_rgb_scale(depth: f64) -> Color {
    let descreet_depth = (depth * LIGHT_RGB_SCALE_ITERS as f64) as u32;
    match descreet_depth {
           0 ...  255 => Color::RGB(
            u8::MAX, 
            descreet_depth as u8, 
            (255 - descreet_depth) as u8,
        ),
         256 ...  511 => Color::RGB(
            (511 - descreet_depth) as u8,
            u8::MAX,
            (descreet_depth - 256) as u8, 
        ),
         512 ...  767 => Color::RGB(
            (descreet_depth - 512) as u8, 
            (767 - descreet_depth) as u8,
            u8::MAX,
        ),
        LIGHT_RGB_SCALE_ITERS => Color::RGB(u8::MAX, 0u8, u8::MAX),
        _ => Color::Black,
    }
}

pub fn orange(depth: f64) -> Color {
    let inverse_depth = 1f64 - depth;
    Color::RGB(
        (u8::MAX as f64 * inverse_depth) as u8,
        (u8::MAX as f64 * 0.6f64 * inverse_depth) as u8,
        0u8,
    )
}

pub fn purple(depth: f64) -> Color {
    let inverse_depth = 1f64 - depth;
    Color::RGB(
        (u8::MAX as f64 * 0.7 * inverse_depth) as u8,
        (u8::MAX as f64 * 0.2f64 * inverse_depth) as u8,
        (u8::MAX as f64 * inverse_depth) as u8,
    )
}
