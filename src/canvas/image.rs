use crate::Color;
use crate::Canvas;
use std::fs;
use std::fs::File;
use colored::*;

pub struct ImageCanvas {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

impl ImageCanvas {
    pub fn convert_to_ppm(&self, path: &str, file_name: &str) -> Result<File, std::io::Error> {
        let mut temp = path.to_string().clone();
        temp.push_str(file_name);
        let image = File::create(temp.clone());
        println!("{}{}", "Writing ".italic().bold(), temp);

        let mut content = String::new();
        //Header
        content.push_str("P3\n");
        content.push_str(self.width.to_string().as_str());
        content.push(' ');
        content.push_str(self.height.to_string().as_str());
        content.push_str("\n255\n");

        for i in 0..self.height {
            for j in 0..self.width {
                let (mut red, mut green, mut blue) = Color::color_into_pixel(&self.data[i * self.width + j]);
                let mut output = String::new();
                output.push_str(red.to_string().as_str());
                output.push(' ');
                output.push_str(green.to_string().as_str());
                output.push(' ');
                output.push_str(blue.to_string().as_str());
                if j < self.width - 1 {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
                content.push_str(output.as_str());
            }
        }
        fs::write(temp, content)?;
        image
    }
}

impl<'a> Canvas for ImageCanvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn clear(&mut self) {
        let data = vec![Color::black(); self.width * self.height];
        self.data = data;
    }

    fn width_as_mut(&mut self) -> &mut usize {
        &mut self.width
    }

    fn height_as_mut(&mut self) -> &mut usize {
        &mut self.height
    }

    fn color_at(&self, row: usize, col: usize) -> &Color {
        /* &self.data[row * self.width + col] */

        &self.data[col * self.width + row]
    }
    fn color_mut_at(&mut self, row: usize, col: usize) -> &mut Color {
        /* &mut self.data[row * self.height + col] */

        &mut self.data[col * self.width + row]
    }

    fn new(width: usize, height: usize) -> Self {
        ImageCanvas {
            data: vec![Color::black(); width * height],
            width,
            height,
        }
    }
}
