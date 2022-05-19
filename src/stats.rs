use crate::colors;

pub fn get_colors_counts(original: &[u32]) -> Vec<u32> {
    let mut colors_count = vec![0; 1 << 24];

    for &number in original {
        colors_count[number as usize] += 1;
    }

    colors_count
}

pub fn dist(p1: u32, p2: u32) -> u32 {
    let [x1, x2, x3] = colors::u32_to_u8s(p1);
    let [y1, y2, y3] = colors::u32_to_u8s(p2);

    ((x1 as i16 - y1 as i16).abs() + (x2 as i16 - y2 as i16).abs() + (x3 as i16 - y3 as i16).abs())
        as u32
}

pub fn noise_part(p: u32) -> u32 {
    let [x, y, z] = colors::u32_to_u8s(p);

    x as u32 + y as u32 + z as u32
}

pub fn dist_squared(p1: u32, p2: u32) -> u32 {
    let dist = dist(p1, p2);

    dist * dist
}

pub fn quant_area_avg(group: &[usize]) -> u32 {
    let mut sum_r = 0;
    let mut sum_g = 0;
    let mut sum_b = 0;

    for &color in group {
        let [r, g, b] = colors::u32_to_u8s(color as u32);

        sum_r += r as u32;
        sum_g += g as u32;
        sum_b += b as u32;
    }

    sum_r /= group.len() as u32;
    sum_g /= group.len() as u32;
    sum_b /= group.len() as u32;

    colors::u8s_to_u32([sum_r as u8, sum_g as u8, sum_b as u8])
}
