use clap::{App, load_yaml};
use image::{ImageBuffer, Luma, RgbImage};

fn is_bright(noise_color: &Luma<u8>, picture_color: &Luma<u8>) -> bool {
    let noise_luma = noise_color.0;
    let picture_luma = picture_color.0;
    if picture_luma[0] > noise_luma[0] {
        true
    } else {
        false
    }
}

fn wrap(m: u32, n: u32) -> u32 {
    return n % m;
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();

    let old_img = image::open(input_file).unwrap();
    let mut old_img = old_img.grayscale();
    let old_img = old_img.as_mut_luma8().unwrap();
    let (old_width, old_height) = old_img.dimensions();

    let noise_img = image::open("img/noise.png").unwrap();
    let mut noise_img = noise_img.grayscale();
    let noise_img = noise_img.as_mut_luma8().unwrap();
    let (noise_width, noise_height) = noise_img.dimensions();

    let _new_img: RgbImage = ImageBuffer::new(old_width, old_height);

    let _new_img = ImageBuffer::from_fn(old_width, old_height, |x, y| {
        let wrap_x = wrap(noise_width, x);
        let wrap_y = wrap(noise_height, y);

        let noise_pixel = noise_img.get_pixel_mut(wrap_x, wrap_y);
        let old_pixel = old_img.get_pixel_mut(x, y);

        if is_bright(noise_pixel, old_pixel) {
            Luma([255u8])
        } else {
            Luma([0u8])
        }
    });

    _new_img.save(&output_file).unwrap();
    println!("File saved to {}", &output_file);
}
