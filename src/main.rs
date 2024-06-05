mod args;

use args::Args;
use image::{
    imageops::FilterType::Triangle,
    io::Reader,
    DynamicImage,
    GenericImageView,
    ImageFormat,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::new();
    println!("{:?}", args);

    let (image_1, image_1_format) = find_image_from_path(&args.image_1)?;
    let (image_2, image_2_format) = find_image_from_path(&args.image_2)?;

    if image_1_format != image_2_format {
        return Err(Box::new(ImageDataErrors::DifferentImageFormats));
    }

    let (image_1, image_2) = standardize_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined_data = combine_images(image_1, image_2);

    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        &output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format,
    )?;

    Ok(())
}

#[derive(Debug)]
enum ImageDataErrors {
    BufferTooSmall,
    DifferentImageFormats,
}

impl std::fmt::Display for ImageDataErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ImageDataErrors {}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = (width * height * 4) as usize;
        let buffer: Vec<u8> = vec![0u8; buffer_capacity];
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

fn find_image_from_path(path: &str) -> Result<(DynamicImage, ImageFormat), Box<dyn Error>> {
    let image_reader = Reader::open(path)?;
    let image_format = image_reader.format().ok_or("Failed to get image format")?;
    let image = image_reader.decode()?;
    Ok((image, image_format))
}

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}\n", width, height);
    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    if pix_1 < pix_2 {
        dim_1
    } else {
        dim_2
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(&vec_1, &vec_2)
}

fn alternate_pixels(vec_1: &[u8], vec_2: &[u8]) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data[i..i + 4].copy_from_slice(&vec_1[i..i + 4]);
        } else {
            combined_data[i..i + 4].copy_from_slice(&vec_2[i..i + 4]);
        }
        i += 4;
    }

    combined_data
}

// cargo build --release
// ./target/release/image_combiner images/image1.png images/image2.png images/output.png
