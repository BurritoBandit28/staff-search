use std::fs::File;
use std::io::{Read, Write};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::sys::Font;
use sdl2::video::WindowContext;
use crate::{get_name, get_image};
use crate::resource_location::ResourceLocation;
use crate::vector2::Vector2;

#[derive(Clone)]
pub struct StaffMember {

    coords : Vector2<i32>,
    acceleration : Vector2<f32>,
    velocity : Vector2<f32>,
    squish : Vector2<f32>,
    name : String
}

impl StaffMember {

    pub fn create<'a>(texture_creator: &'a TextureCreator<WindowContext>, texture: &mut Texture<'a>) -> Self {

        let (name,usrname) = get_name().unwrap();


        println!("{}", name);
        let mut image_file = get_image(usrname.clone()).expect("Failed to get image!");
        let bytes : [u8;1024] =  [0u8;1024];
        image_file.write_all(&bytes);

        let newtexture = texture_creator.load_texture(format!("./tmp/{}.png",usrname)).expect("Failed to read image bytes");

        *texture = newtexture;

        Self {
            coords: Vector2::new(50,50),
            acceleration: Vector2::new(0.0,0.0),
            velocity: Vector2::new(0.0,0.0),
            squish: Vector2::new(0.0,0.0),
            name,
        }

    }

    pub fn render(&mut self, mut canvas: &mut WindowCanvas, texture: &Texture, font: &sdl2::ttf::Font) {

        canvas.copy(texture, None, None);

        let surface = font.render(&self.name).blended(Color::RGBA(255, 0, 0, 255)).expect("what");
        let texture_creator =  canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(surface).expect("Failed");
        canvas.copy(&texture, None, None);

    }

}
