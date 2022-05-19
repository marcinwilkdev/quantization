mod colors;
mod stats;

use image::Rgb;
use std::path::Path;

const EPSILON: f64 = 0.01;

pub fn quantize(input: &Path, output: &Path, colors: usize) {
    let img = image::open(input).expect("couldn't open an image");
    let mut rgb_img = img.to_rgb8();

    let colors_count = if colors == 0 { 0 } else { 1 << (colors - 1) };

    let (new_pixels, mean_squared_error, signal_to_noise) =
        linnde_and_transform(colors_count, EPSILON, &rgb_img.pixels().collect::<Vec<_>>());

    for (i, pixel) in rgb_img.pixels_mut().enumerate() {
        *pixel = new_pixels[i].into();
    }

    rgb_img.save(output).expect("couldnt save image");

    println!("Mean squared error: {}", mean_squared_error);
    println!("Noise: {}", signal_to_noise);
}

pub fn linnde_and_transform(
    num_colors: u32,
    epsilon: f64,
    img_pixels: &[&Rgb<u8>],
) -> (Vec<[u8; 3]>, f64, f64) {
    let img_pixels = img_pixels
        .iter()
        .map(|img_color| colors::color_to_u32(img_color))
        .collect::<Vec<_>>();

    let mut quant_colors = colors::gen_random_colors(num_colors);

    let mut curr_error = 0;

    let img_colors_counts = stats::get_colors_counts(&img_pixels); // for probability calculating

    let mut img_to_quant_color_map = vec![0; img_colors_counts.len()];

    let mut signal_to_noise;

    loop {
        let mut quant_areas = vec![vec![]; num_colors as usize];

        for (img_color, _) in img_colors_counts
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
        {
            let (_, img_color, quant_color_index) = quant_colors
                .iter()
                .enumerate()
                .map(|(quant_color_index, &quant_color)| {
                    (
                        stats::dist(img_color as u32, quant_color),
                        img_color,
                        quant_color_index,
                    )
                })
                .min_by(|(d1, _, _), (d2, _, _)| d1.cmp(d2))
                .unwrap();

            quant_areas[quant_color_index].push(img_color);
            img_to_quant_color_map[img_color] = quant_colors[quant_color_index];
        }

        let mut sum_distances = 0;
        signal_to_noise = 0;

        for area_index in 0..num_colors {
            for &img_color in &quant_areas[area_index as usize] {
                sum_distances +=
                    stats::dist_squared(quant_colors[area_index as usize], img_color as u32)
                        * img_colors_counts[img_color];

                signal_to_noise +=
                    stats::noise_part(img_color as u32) * img_colors_counts[img_color];
            }
        }

        let mean_squared_error =
            ((sum_distances as f64 - curr_error as f64) / sum_distances as f64).abs();

        curr_error = sum_distances;

        if mean_squared_error < epsilon {
            break;
        }

        println!("error: {} eps: {}", mean_squared_error, epsilon);

        for area_index in 0..num_colors {
            if quant_areas[area_index as usize].len() > 0 {
                quant_colors[area_index as usize] =
                    stats::quant_area_avg(&quant_areas[area_index as usize]);
            }
        }
    }

    let final_colors = img_pixels
        .iter()
        .map(|&img_color| img_to_quant_color_map[img_color as usize])
        .map(|quant_color| colors::u32_to_u8s(quant_color))
        .collect::<Vec<_>>();

    let mean_squared_error = curr_error as f64 / img_pixels.len() as f64;

    (
        final_colors,
        mean_squared_error,
        10.0 * ((signal_to_noise as f64 / img_pixels.len() as f64) / mean_squared_error).log10(),
    )
}
