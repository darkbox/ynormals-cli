extern crate clap;
extern crate image;

use image::{GenericImageView, GenericImage, DynamicImage};
use clap::{App, Arg};

fn main() {
    let matches = App::new("yNormals")
        .version("0.1.0")
        .author("Rafa G.M. <rgtresd@gmail.com>")
        .about("Flip image channels (rgba)")
        .arg(Arg::with_name("IMAGE")
            .index(1)
            .required(true)
            .takes_value(true)
            .help("Filename"))
        .arg(Arg::with_name("FLIP_CHANNELS")
            .long("flip")
            .takes_value(true)
            .help("Flags to flip channels [rgba]"))
        .arg(Arg::with_name("VERBOSE")
            .long("verbose")
            .short("v")
            .help("Verbose mode"))
        .get_matches();

    let path = matches.value_of("IMAGE").unwrap();
    let save_path = path.clone();
    let verbose_mode = match matches.value_of("VERBOSE") {
        Some(_) => true,
        _ => false
    };

    if let Ok(mut img) = image::open(path) {
        if verbose_mode {
            println!("input.....: {:?}", path);
            println!("output....: {:?}", save_path);
            println!("dimensions: {:?}", img.dimensions());
            println!("color.....: {:?}", img.color());
        }

        flip_channels(&mut img, flip_flags_to_u8(matches.value_of("FLIP_CHANNELS").unwrap_or("0")));
        img.save(save_path).unwrap();
    } else {
        println!("Could not open the image");
    }
}

fn flip_flags_to_u8(flags: &str) -> u8 {
    let mut value: u8 = 0;

    if flags.chars().any(|x| {x.eq(&'r')}) {
        value = value | 0b0001;
    }
    if flags.chars().any(|x| {x.eq(&'g')}) {
        value = value | 0b0010;
    }
    if flags.chars().any(|x| {x.eq(&'b')}) {
        value = value | 0b0100;
    }
    if flags.chars().any(|x| {x.eq(&'a')}) {
        value = value | 0b1000;
    }

    value
}

fn flip_channels(img: &mut DynamicImage, flags: u8){
    for x in 0..img.width() {
        for y in 0..img.height(){
            let image::Rgba(data) = img.get_pixel(x, y);
            let mut r = data[0];
            let mut g = data[1];
            let mut b = data[2];
            let mut a = data[3];

            if 1 == flags & 0b0001 { r = 255 - r; }
            if 2 == flags & 0b0010 { g = 255 - g; }
            if 4 == flags & 0b0100 { b = 255 - b; }
            if 8 == flags & 0b1000 { a = 255 - a; }

            let new_pixel = image::Rgba([r, g, b, a]);
            img.put_pixel(x, y, new_pixel);
        }
    }

}