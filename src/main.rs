extern crate image;

mod action;
mod cli;
use image::*;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::process::exit;
use std::sync::mpsc;

fn main() {
    let (io, settings, image_names) = cli::parse();

    // Load images
    let mut images: HashMap<String, mpsc::Receiver<DynamicImage>> = HashMap::new();
    for image_name in image_names {
        if images.contains_key(&image_name) {
            continue;
        }
        let (s, r) = mpsc::channel();
        let i_n = image_name.clone();
        std::thread::spawn(move || {
            s.send(
                image::open(&i_n)
                    .map_err(|e| {
                        eprintln!("{}", e);
                    })
                    .unwrap_or_else(|_| {
                        eprintln!("Aborting because one or more errors while loading image");
                        exit(2)
                    }),
            )
            .unwrap();
        });
        images.insert(image_name, r);
    }

    // Use extension of outfile as default, can be overwritten with format: action
    let outname = io.1.clone().to_owned();
    let gutted_outname: Vec<&str> = outname.split(".").collect();
    let out_format = match gutted_outname[gutted_outname.len() - 1] {
        "png" => ImageOutputFormat::Png,
        "jpg" => ImageOutputFormat::Jpeg(100),
        "jpeg" => ImageOutputFormat::Jpeg(100),
        "bmp" => ImageOutputFormat::Bmp,
        "gif" => ImageOutputFormat::Gif,
        "ico" => ImageOutputFormat::Ico,
        &_ => ImageOutputFormat::Png,
    };

    let (image, out_format) =
        action::apply_actions(&io.0, out_format, settings.actions, settings.flags, images);

    match io.1.as_ref() {
        "stdout" => {
    	    let mut buffer = Vec::new();
    	    image.write_to(&mut std::io::Cursor::new(&mut buffer), out_format).unwrap_or_else(|e| {
        	eprintln!("Failed to encode image: {}", e);
        	exit(2)
    	    });
    	    std::io::stdout().write_all(&buffer).unwrap_or_else(|e| {
        	eprintln!("Failed to write image to stdout: {}", e);
        	exit(2)
    	    });
        },
        _ => image
            .write_to(
                &mut File::create(&io.1).unwrap_or_else(|_| {
                    eprintln!("Outfile {} not found", io.1);
                    exit(2)
                }),
                out_format,
            )
            .unwrap_or_else(|e| {
                eprintln!("Failed to save image: {}", e);
                exit(2)
            }),
    }
}
