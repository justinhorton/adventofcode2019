extern crate image;

use std::cmp::min;
use std::io::Error;

const INPUT: RawData = RawData {
    width: 25,
    height: 6,
    data: include_str!("../day8.txt"),
};
const PT2_IMG_PATH: &str = "./day8/part2-output.png";

fn main() {
    let image_data = INPUT.parse();

    println!("Day 8-1: {}", calc_day8_part1(&image_data));
    match image_data.save_image(PT2_IMG_PATH) {
        Ok(_) => println!("Day 8-2: result saved to {}", PT2_IMG_PATH),
        Err(_) => println!("Day 8-2: failed to generate {}", PT2_IMG_PATH),
    }
}

fn calc_day8_part1(data: &ImageData) -> u32 {
    let mut bins_for_layer_with_fewest_zeros: [u32; 10] = data.get_binned_digits_for_layer(0);
    for layer_num in 1..data.layers {
        let binned_digits = data.get_binned_digits_for_layer(layer_num);
        if binned_digits[0] < bins_for_layer_with_fewest_zeros[0] {
            bins_for_layer_with_fewest_zeros = binned_digits;
        }
    }

    bins_for_layer_with_fewest_zeros[1] * bins_for_layer_with_fewest_zeros[2]
}

struct RawData<'a> {
    height: usize,
    width: usize,
    data: &'a str,
}

impl RawData<'_> {
    fn parse(&self) -> ImageData {
        let mut parsed_data: Vec<u32> = Vec::new();
        let mut chars = self.data.trim().chars();

        while let Some(Some(char_as_int)) = chars.next().map(|c| c.to_digit(10)) {
            parsed_data.push(char_as_int);
        }
        ImageData {
            height: self.height,
            width: self.width,
            layers: parsed_data.len() / (self.height * self.width),
            data: parsed_data,
        }
    }
}

#[derive(Debug)]
struct ImageData {
    height: usize,
    width: usize,
    layers: usize,
    data: Vec<u32>,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    // TODO: Could get rid of this in favor of image::Rgb
    Black,
    White,
    Transparent,
}

impl Color {
    fn from(d: &u32) -> Color {
        return match d {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            x => panic!("Invalid data: {}", x),
        };
    }
}

impl ImageData {
    fn get_layer(&self, i: usize) -> Option<&[u32]> {
        if i >= self.layers {
            return None;
        } else {
            let layer_start: usize = self.width * self.height * i;
            let layer_end: usize = min(self.width * self.height * (i + 1), self.data.len());
            return Some(&self.data[layer_start..layer_end]);
        }
    }

    fn get_binned_digits_for_layer(&self, i: usize) -> [u32; 10] {
        let mut bins: [u32; 10] = [0; 10];
        self.get_layer(i).map(|l| {
            for d in l.iter() {
                bins[*d as usize] += 1;
            }
        });
        bins
    }

    fn decode(&self) -> Vec<Color> {
        let mut decoded_image = vec![Color::Transparent; self.width * self.height];

        for layer in (0..self.layers).rev() {
            self.get_layer(layer).map(|l| {
                let mut img_i: usize = 0;

                for d in l.iter() {
                    let color = Color::from(d);
                    let prior_layer_color = decoded_image[img_i];
                    let new_color = match color {
                        Color::Transparent => prior_layer_color,
                        _ => color,
                    };
                    decoded_image[img_i] = new_color;
                    img_i += 1;
                }
            });
        }
        decoded_image
    }

    fn save_image(&self, png_output_path: &str) -> Result<(), Error> {
        let decoded_img_colors = self.decode();

        let mut color_i = 0;
        let mut img_buf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (_x, _y, pixel) in img_buf.enumerate_pixels_mut() {
            let color = decoded_img_colors[color_i];
            *pixel = match color {
                Color::White => image::Rgb([255, 255, 255]),
                Color::Black => image::Rgb([0, 0, 0]),
                _ => panic!("Bad image: output has transparent pixel"),
            };
            color_i += 1;
        }
        img_buf.save(png_output_path)
    }
}
