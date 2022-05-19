use rand::prelude::*;
use image::Rgb;

pub fn gen_random_colors(num_colors: u32) -> Vec<u32> {
    let mut color = vec![false; 1 << 24];
    let mut generated_colors = 0;

    while generated_colors < num_colors {
        let generated = rand_24bits();

        if !color[generated as usize] {
            color[generated as usize] = true;
            generated_colors += 1;
        }
    }

    color
        .iter()
        .enumerate()
        .filter(|(_, &c)| c)
        .map(|(i, _)| i as u32)
        .collect()
}

pub fn color_to_u32(color: &Rgb<u8>) -> u32 {
    u8s_to_u32([color[0], color[1], color[2]])
}

pub fn rand_24bits() -> u32 {
    let mut num: u32 = thread_rng().gen();
    num >>= 8;
    num
}

pub fn u32_to_u8s(mut number: u32) -> [u8; 3] {
    let c = (number & 0b1111_1111) as u8;
    number >>= 8;
    let b = (number & 0b1111_1111) as u8;
    number >>= 8;
    let a = (number & 0b1111_1111) as u8;

    [a, b, c]
}

pub fn u8s_to_u32(u8s: [u8; 3]) -> u32 {
    ((u8s[0] as u32) << 16) + ((u8s[1] as u32) << 8) + (u8s[2] as u32)
}
