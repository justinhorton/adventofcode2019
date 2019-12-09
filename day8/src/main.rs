extern crate image;

use std::io::Error;

const INPUT: RawData = RawData {
    width: 25,
    height: 6,
    data: include_str!("../day8.txt"),
};
const PT2_IMG_PATH: &str = "./day8/part2-output.png";

const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSPARENT:u32 = 2;

fn main() {
    let image_data = INPUT.parse();

    println!("Day 8-1: {}", calc_day8_part1(&image_data));
    match image_data.save_image(PT2_IMG_PATH) {
        Ok(_) => println!("Day 8-2: result saved to {}", PT2_IMG_PATH),
        Err(_) => println!("Day 8-2: failed to generate {}", PT2_IMG_PATH),
    }
}

fn calc_day8_part1(image_data: &ImageData) -> usize {
    let img_size = image_data.width * image_data.height;

    let (mut min_index, mut zeros_ones_twos) = (0, [img_size, 0, 0]);
    for (layer_index, layer) in image_data.data.chunks(img_size).enumerate() {
        let zeros = layer.iter().filter(|it| **it == 0).count();
        if zeros < zeros_ones_twos[0] {
            min_index = layer_index;
            zeros_ones_twos[0] = zeros;
        }
    }

    // fill in the ones and twos for the min row
    let layer_slice = &image_data.data[(img_size * min_index)..(img_size * (min_index + 1))];
    for v in layer_slice.iter() {
        zeros_ones_twos[*v as usize] += 1;
    }

    zeros_ones_twos[1] * zeros_ones_twos[2]
}

struct RawData<'a> {
    height: usize,
    width: usize,
    data: &'a str,
}

impl RawData<'_> {
    fn parse(&self) -> ImageData {
        let parsed_data: Vec<u32> = self.data.trim().chars()
            .map(|c| c.to_digit(10))
            .map(|o| o.unwrap())
            .collect();
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

impl ImageData {
    fn decode(&self) -> Vec<u32> {
        let mut decoded_image = vec![TRANSPARENT; self.width * self.height];

        self.data.chunks(self.width * self.height).for_each(|layer| {
            for (i, layer_color) in layer.iter().enumerate() {
                if decoded_image[i] == TRANSPARENT {
                    decoded_image[i] = *layer_color;
                }
            };
        });

        decoded_image
    }

    fn save_image(&self, png_output_path: &str) -> Result<(), Error> {
        let decoded = self.decode();

        let mut img_buf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            *pixel = match decoded[(y * self.width as u32 + x) as usize] {
                WHITE => image::Rgb([255, 255, 255]),
                BLACK => image::Rgb([0, 0, 0]),
                _ => panic!("Bad image: output has transparent pixel")
            };
        }
        img_buf.save(png_output_path)
    }
}
