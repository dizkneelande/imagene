use crate::action::Action;
use crate::action::Action::*;
use crate::action::{Direction, Flag, Orientation};
use image::ImageOutputFormat;
use std::collections::HashMap;
use std::process::exit;

use colored::*;
use std::env;

pub struct Settings {
    pub actions: Vec<Action>,
    pub flags: HashMap<Flag, bool>,
}

pub fn parse() -> ((String, String), Settings, Vec<String>) {
    let mut settings = Settings {
        actions: Vec::new(),
        flags: HashMap::new(),
    };
    let args: Vec<String> = env::args().collect();

    let help_message = format!(
            "Syntax:
    {imagene} {o}infile{c} ...{o}flag{c}... ...{o}action{c}:{o}value{c}... {o}outfile{c}

Available Actions:
    brightness:{o}int{c}           {comment} Increase brightness by percent
    contrast:{o}int{c}             {comment} Increase contrast by percent
    blur:{o}float{c}               {comment} Add gaussian blur by sigma (recommended 1-20)
    unsharpen:{o}float,int{c}      {comment} Add unsharpen mask with float being sigma and int being threshold
    invert:{o}true/false{c}        {comment} Invert colors of image
    flip:{o}v/h{c}                 {comment} Flip image v for vertically or h for horizontally
    rotate:{o}left/right/down{c}   {comment} Rotate an image by 90,180,270 degrees
    resize:{o}int,int{c}           {comment} Resize an image, leave one of the ints empty to auto scale it
    crop:{o}int,int,int,int{c}     {comment} Crop an image (x,y,width,height)
    append:{o}string,left/under{c} {comment} Add another image next to source image
    text:{o}string,(x:y),(r:g:b:a),(font:scale){c}   {comment} Add text onto an image, \"font\" is a path to .ttf file and values are floats
    format:{o}string{c}            {comment} Specify output image format
    format:{o}jpg,int{c}           {comment} For JPG, also specify quality

Available Flags:
    lanczos3   {comment} Use a slower but possibly higher quality algorithm

Examples:
     {comment} Increases the contrast of the original image by 20% and adds an extra image next to it
     {imagene} {infile} contrast:20 append:extra_image.png,left {outfile}

     {comment} Set width to 2000, automatically scales height to keep aspect ratio and output to STDOUT
     {imagene} {infile} resize:2000,0 stdout

     {comment} Overwrites an image with increased contrast
     {imagene} {infile} contrast:2 {infile} ",
            imagene = "imagene".green(),
            infile = "in_file.png".blue(),
            outfile = "out_file.png".blue(),
            comment = "->".blue(),
            o = "<".green(),
            c = ">".green()
        );

    if args.len() <= 3 {
        println!("{}", help_message);
        exit(0);
    }
    if &args[1] == "--help" || &args[1] == "-h" || &args[1] == "/h" || &args[1] == "help" {
        println!("{}", help_message);
        exit(0);
    }

    let infile = &args[1];
    let outfile = &args[args.len() - 1];
    if outfile != "stdout" {
        println!("Using infile {} and outfile {}", infile, outfile);
    };

    let mut images: Vec<String> = Vec::new();
    images.push(infile.to_owned());

    for arg in &args[2..args.len() - 1] {
        let step = split_kv(arg);

        match step {
            Ok((k, v)) => {
                // Key:Value based argument
                settings.actions.push(match k {
                    "invert" => {
                        if v == "true" || v == "1" {
                            Invert
                        } else {
                            continue;
                        }
                    }
                    "contrast" => Contrast(v.to_owned().parse::<f32>().unwrap_or_else(|_| {
                        eprintln!("{}: Invalid value for {}", k, v);
                        exit(2)
                    })),
                    "brightness" => Brightness(v.to_owned().parse::<i32>().unwrap_or_else(|_| {
                        eprintln!("{}: Invalid value for {}", k, v);
                        exit(2)
                    })),
                    "blur" => Blur(v.to_owned().parse::<f32>().unwrap_or_else(|_| {
                        eprintln!("{}: Invalid value for {}", k, v);
                        exit(2)
                    })),
                    "crop" => {
                        let crop_arguments: Vec<&str> = v.split(",").collect();
                        if crop_arguments.len() != 4 {
                            eprintln!("Wrong amount of arguments for crop, i need \"x,y,w,h\"");
                            exit(2);
                        }
                        let convert = |s: &str| {
                            s.to_owned().parse::<u32>().unwrap_or_else(|_| {
                                eprintln!("{}: Invalid value for {}", s, k);
                                exit(2);
                            })
                        };
                        Crop(
                            convert(crop_arguments[0]),
                            convert(crop_arguments[1]),
                            convert(crop_arguments[2]),
                            convert(crop_arguments[3]),
                        )
                    }
                    "rotate" => Rotate(match v.as_str() {
                        "down" => Direction::Down,
                        "left" => Direction::Left,
                        "right" => Direction::Right,
                        _ => {
                            eprintln!("Invalid value for rotate, use left right or down");
                            exit(2)
                        }
                    }),
                    "flip" => match v.as_str() {
                        "v" => Flip(Orientation::Vertical),
                        "h" => Flip(Orientation::Horizontal),
                        _ => {
                            eprintln!("Invalid value for flip, use v or h");
                            exit(2)
                        }
                    },
                    "unsharpen" => {
                        let unsharp_arguments: Vec<&str> = v.split(",").collect();
                        if unsharp_arguments.len() != 2 {
                            {
                                eprintln!("Wrong amount of arguments for unsharpen")
                            };
                            exit(2)
                        };
                        let convert = |s: &str| {
                            s.to_owned().parse::<f32>().unwrap_or_else(|_| {
                                eprintln!("{}: Invalid value for {}", s, k,);
                                exit(2)
                            })
                        };
                        Unsharpen(
                            convert(unsharp_arguments[0]),
                            convert(unsharp_arguments[1]) as i32,
                        )
                    }
                    "resize" => {
                        let resize_arguments: Vec<&str> = v.split(",").collect();
                        if resize_arguments.len() != 2 {
                            eprintln!("Wrong amount of arguments for resize");
                            exit(2);
                        };
                        Scale(
                            resize_arguments[0]
                                .to_owned()
                                .parse::<u32>()
                                .unwrap_or_else(|_| {
                                    eprintln!("{}: Invalid value for {}", resize_arguments[0], v,);
                                    exit(2)
                                }),
                            resize_arguments[1]
                                .to_owned()
                                .parse::<u32>()
                                .unwrap_or_else(|_| {
                                    eprintln!("{}: Invalid value for {}", resize_arguments[1], v,);
                                    exit(2);
                                }),
                        )
                    }
                    "append" => {
                        let append_arguments: Vec<&str> = v.split(",").collect();
                        if append_arguments.len() != 2 {
                            eprintln!("Wrong amount of arguments for append");
                            exit(2);
                        };
                        images.push(append_arguments[0].to_owned());
                        Append(
                            append_arguments[0].to_owned(),
                            match append_arguments[1] {
                                "left" => Direction::Left,
                                "right" => Direction::Right,
                                "down" => Direction::Down,
                                "under" => Direction::Down,
                                "up" => Direction::Up,
                                "over" => Direction::Up,
                                _ => {
                                    eprintln!("Second parameter invalid for append");
                                    exit(2)
                                }
                            },
                        )
                    }
                    "format" => {
                        let format_arguments: Vec<&str> = v.split(",").collect();
                        if format_arguments.len() == 2 {
                            Format(ImageOutputFormat::Jpeg(
                                format_arguments[1]
                                    .to_owned()
                                    .parse::<u8>()
                                    .unwrap_or_else(|_| {
                                        eprintln!(
                                            "{}: Invalid format for {}",
                                            format_arguments[1], format_arguments[0]
                                        );
                                        exit(2);
                                    }),
                            ))
                        } else {
                            Format(match format_arguments[0] {
                                "png" => ImageOutputFormat::Png,
                                "gif" => ImageOutputFormat::Gif,
                                "bmp" => ImageOutputFormat::Bmp,
                                "ico" => ImageOutputFormat::Ico,
                                &_ => {
                                    eprintln!("Invalid value for format");
                                    exit(2)
                                }
                            })
                        }
                    }
                    "watermark" | "write" | "text" => {
                        let text_arguments: Vec<&str> = v.split(",").collect();
                        if text_arguments.len() != 4 {
                            eprintln!("Wrong amount of arguments for {:?}", text_arguments);
                            exit(2);
                        };
                        let content = text_arguments[0];
                        let position = {
                            let sub = sub_category_parse(text_arguments[1], 2).expect(
                                &format!("Unexpected format for position of watermark/write/text. Expected (0.3 : 0.5), got {}", text_arguments[1]));
                            let sub: Vec<f32> = sub.into_iter()
                                .map(|s| s.parse()
                                     .expect("Invalid value for position of watermark/write/text. Expected float"))
                                .collect();
                            (sub[0], sub[1])
                        };
                        let color_rgba = {
                            let sub = sub_category_parse(text_arguments[2], 4).expect(
                                &format!("Unexpected format for color of watermark/write/text. Expected (1.0 : 0.0 : 0.0 : 0.5), got {}", text_arguments[2]));
                            let sub: Vec<f32> = sub.into_iter()
                                .map(|s| s.parse()
                                     .expect("Invalid value for position of watermark/write/text. Expected float"))
                                .collect();
                            (sub[0], sub[1], sub[2], sub[3])
                        };
                        let font = {
                            let sub = sub_category_parse(text_arguments[3], 2).expect(
                                &format!("Unexpected format for color of watermark/write/text. Expected (1.0 : 0.0 : 0.0 : 0.5), got {}", text_arguments[2]));
                            (sub[0].to_owned(), sub[1].parse::<f32>().expect("Invalid value for position of watermark/write/text. expected float"))
                        };
                        Watermark(
                            content.to_owned(),
                            position,
                            color_rgba,
                            font,
                        )
                    }
                    &_ => {
                        eprintln!("{}: action not found", k);
                        exit(2);
                    }
                });
            }
            Err(err) => {
                // Flag based argument
                let name: &str = arg.as_ref();
                match name {
                    "lanczos3" => settings.flags.insert(Flag::Lanczos3, true),
                    &_ => {
                        eprintln!("Unrecognized argument \"{}\"\n{}", arg, err);
                        exit(2);
                    }
                };
            }
        }
    }

    ((infile.to_owned(), outfile.to_owned()), settings, images)
}

fn split_kv(s: &str) -> Result<(&str, String), String> {
    let split: Vec<&str> = s.split(":").collect();
    if split.len() < 2 {
        let mut r = String::from("Parse error on argument: ");
        r.push_str(s);
        Err(r)
    } else {
        let mut value = String::new();
        for string in split[1..].into_iter() {
            value.push_str(string);
            value.push_str(":");
        }
        Ok((
            &split[0],
            if value.chars().last().unwrap() == ':' {
                value[0..value.len() - 1].to_owned()
            } else {
                value
            },
        ))
    }
}

fn sub_category_parse(c: &str, args: usize) -> Result<Vec<String>, &str> {
    let sub_category = escape(String::from(c));

    let parsed: Vec<String> = sub_category
        .replace("(", "")
        .replace(")", "")
        .replace(" ", "")
        .split(":")
        .map(|s| s.to_owned())
        .collect();
    if parsed.len() != args {
        Err("Wrong amount of arguments")
    } else {
        Ok(parsed)
    }
}

fn escape(c: String) -> String {
    let split: Vec<&str> = c.split("\\").collect();
    if split.len() == 1 {
        return c;
    };
    split[1..]
        .into_iter()
        .map(|s| {
            s.chars()
                .next()
                .map(|c| String::from(&s[c.len_utf8()..]))
                .unwrap()
        })
        .collect()
}

pub fn flag_is_enabled(v: Option<&bool>) -> bool {
    *v.unwrap_or(&false)
}
