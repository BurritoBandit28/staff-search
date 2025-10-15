mod render;
mod vector2;
mod resource_location;

use std::fmt::format;
use std::fs;
use std::fs::{ File};
use std::io::{Read, Write, copy, Cursor};
use std::time::Duration;
use dialog::DialogBox;
use image::ImageReader;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::render::StaffMember;

fn main() -> std::io::Result<()> {

    // create the ./tmp directory - where proffessor images go
    fs::create_dir("./tmp");

    /*
    let (name,usrname) = get_name().unwrap();

    println!("{}", name);
    get_image(usrname);
     */
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    
    
    let window = video_subsystem.window("Name Fetcher", 800, 600)
        .position_centered()
        .vulkan()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut current_texture = texture_creator.load_texture("./assets/default.png").expect("Error initiating texture");

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.copy(&current_texture, None, None);
    canvas.present();

    let mut staffmember : StaffMember = StaffMember::create(&texture_creator, &mut current_texture);

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let mut font = ttf_context.load_font("./assets/JetBrainsMono-Regular.ttf", 128).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {


        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::R),..} => {

                    staffmember = StaffMember::create(&texture_creator, &mut current_texture)

                }
                _ => {}
            }
        }

        staffmember.render(&mut canvas, &current_texture, &font);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}



fn get_name() -> anyhow::Result<(String,String)> {



    let mut username = String::new();
    let name = dialog::Input::new("Please enter your username")
        .title("Enter Username")
        .show()
        .expect("Could not display dialog box");
    match name {
        Some(name) => username = name.to_lowercase(),
        None => println!("Error!"),
    };


    let mut client = reqwest::blocking::Client::builder()
        .proxy(reqwest::Proxy::http("https://152.78.128.51:3128")?)
        .build()?;

    let response = client.get(format!("https://www.ecs.soton.ac.uk/people/{}", username)).send()?;
    let url = format!("{}", response.url());
    let list = url.split("/").collect::<Vec<&str>>();
    let name = list[5];

    Ok((to_title_case(name), username))

}

fn get_image(username : String) -> anyhow::Result<File> {


    if fs::exists(format!("./tmp/{}.png", username)).unwrap() {
        Ok(File::open(format!("./tmp/{}.png", username)).unwrap())
    }
    else {
        let mut client = reqwest::blocking::Client::builder()
            .proxy(reqwest::Proxy::http("https://152.78.128.51:3128")?)
            .build()?;




        let mut response = client.get(format!("https://www.southampton.ac.uk/sites/default/files/styles/max_1300x1300/public/staff/{}.jpg.webp", username)).send()?;

        let mut image = ImageReader::new(Cursor::new(response.bytes().unwrap())).with_guessed_format()?.decode()?;


        image.save(format!("./tmp/{}.png", username));

        Ok(File::open(format!("./tmp/{}.png", username)).unwrap())
    }

}

fn to_title_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}



