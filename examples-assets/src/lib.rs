#![no_std]
#![no_main]
#![allow(warnings)]

use tinybmp::Bmp;

use core::f32::consts::PI;
use libm::{sin, cos};

use embedded_graphics::{
    prelude::{RgbColor, Point, Primitive, Size, DrawTarget, Dimensions, PixelColor},
    image::Image,
    pixelcolor::Rgb565,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder,
    },
    primitives::{Triangle, Rectangle, Circle, RoundedRectangle, PrimitiveStyle, Line}, 
    text::{Alignment, Text},
    Drawable,
};

//rustmas functions

pub fn ferris<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let ferris_data = include_bytes!("../../data/ferris.bmp");
    let ferris = Bmp::from_slice(ferris_data).unwrap();
    Image::new(&ferris, Point::new(87, 140)).draw(display);
    
}

pub fn hat<D>(display: &mut D, pos_x: f64, pos_y: f64)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let default_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::WHITE)
        .build();

    let n = 6.0;
    let d = 71.0;    
    let mut a;
    let mut r;
    let mut x;
    let mut y;
    
    for t in 0..361 {
        a = t as f64 * d * (PI as f64 / 18.0);
        r = 10.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + pos_x) as i32, (y + pos_y) as i32), default_style,  Alignment::Center)
            .draw(display);
    }

    Triangle::new(
        Point::new(pos_x as i32, (pos_y + 8.0) as i32),
        Point::new((pos_x - 32.0) as i32, (pos_y + 41.0) as i32),
        Point::new((pos_x + 35.0) as i32, (pos_y + 41.0) as i32),
    )
    .into_styled(PrimitiveStyle::with_fill(RgbColor::RED))
    .draw(display);

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new((pos_x - 42.0) as i32, (pos_y + 42.0) as i32), Size::new(89, 18)),
        Size::new(10, 10),
    )
    .into_styled(PrimitiveStyle::with_fill(RgbColor::WHITE))
    .draw(display);
    
}

pub fn logo<D>(display: &mut D)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(19, 80), Size::new(95, 95)),
        Size::new(10, 10),
    )
    .into_styled(PrimitiveStyle::with_fill(RgbColor::RED))
    .draw(display);

    let espressif_data = include_bytes!("../../data/espressif.bmp");

    let logo = Bmp::from_slice(espressif_data).unwrap();

    Image::new(&logo, Point::new(30, 89)).draw(display);
}

pub fn tree<D>(display: &mut D)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let tree_style = PrimitiveStyle::with_fill(RgbColor::GREEN);
    
    Triangle::new(
        Point::new(280, 5),
        Point::new(250, 75),
        Point::new(310, 75),
    )
    .into_styled(tree_style)
    .draw(display);

    Triangle::new(
        Point::new(280, 35),
        Point::new(250, 135),
        Point::new(310, 135),
    )
    .into_styled(tree_style)
    .draw(display);

    Triangle::new(
        Point::new(280, 95),
        Point::new(250, 195),
        Point::new(310, 195),
    )
    .into_styled(tree_style)
    .draw(display);

    Rectangle::new(Point::new(275, 196), Size::new(15, 45))
    .into_styled(PrimitiveStyle::with_fill(Rgb565::new(58, 29, 0)))
    .draw(display);    
}

pub fn gift<D>(display: &mut D, pos_x: i32, pos_y: i32)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let gift_data = include_bytes!("../../data/gift.bmp");

    let gift = Bmp::from_slice(gift_data).unwrap();

    Image::new(&gift, Point::new(pos_x, pos_y)).draw(display);
}

pub fn gifts<D>(display: &mut D, pos_x: i32, pos_y: i32)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let gift_data = include_bytes!("../../data/stack_of_gifts.bmp");

    let gifts = Bmp::from_slice(gift_data).unwrap();

    Image::new(&gifts, Point::new(pos_x, pos_y)).draw(display);
}

pub fn snowflake<D>(display: &mut D, x_value: i32, y_value: i32, size: u32)
    where 
        D:DrawTarget<Color = Rgb565>+Dimensions {
    
    Circle::new(Point::new(x_value, y_value), size % 15 + 5)
        .into_styled(PrimitiveStyle::with_fill(RgbColor::WHITE))
        .draw(display);
}

//squares functions

pub fn print_char<D>(display: &mut D, ch: &str, color: Rgb565, x: i32, y: i32)
    where 
        D:DrawTarget<Color = Rgb565>+Dimensions {
            
    Text::with_alignment(ch, Point::new(x, y), 
        MonoTextStyleBuilder::new().font(&FONT_10X20).text_color(color).build(),  Alignment::Center)
        .draw(display);
}

pub fn print_squares<D>(display: &mut D, ch: &str, color: Rgb565, mut start_point: i32, mut edge_x: i32, mut edge_y: i32, direction: i32, constraints: [i32; 3])
    where 
        D:DrawTarget<Color = Rgb565>+Dimensions {
            
    while start_point != constraints[0] && edge_x != constraints[1] && edge_y != constraints[2] {
        for x in start_point..edge_x {
            print_char(display, ch, color, x, start_point);
        }
    
        for y in start_point..edge_y {
            print_char(display, ch, color, edge_x, y);
        }
    
        for x in (start_point..edge_x).rev() {
            print_char(display, ch, color, x, edge_y);
        }
    
        for y in (start_point..edge_y).rev() {
            print_char(display, ch, color, start_point, y);
        }
        start_point += direction * 10;
        edge_x -= direction * 10;
        edge_y -= direction * 10;
    }
}

//lollipop guy functions

pub fn body_part<D>(display: &mut D, ch:&str, modifiers: [f64; 4], len: i32, sign1: i32, sign2: i32)
    where 
        D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let mut vt;
    let mut x;
    let mut y;
            
    for i in 0..len {
        vt = i as f64 / (modifiers[0] * PI as f64);
        x = (vt + (sign1 as f64 * modifiers[1])) * sin(vt);
        y = (vt + (sign2 as f64 * modifiers[1])) *  cos(vt);
        Text::with_alignment(ch, Point::new((x + modifiers[2]) as i32, (y + modifiers[3]) as i32), 
        MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(RgbColor::BLACK)
            .build(), Alignment::Center)
        .draw(display);
    }
}

pub fn hand<D>(display: &mut D, x: i32, y: i32)
    where 
        D:DrawTarget<Color = Rgb565>+Dimensions {
            
    let mut b;
    for a in (x..y).rev() {
        b = a;
        Text::with_alignment("-", Point::new(a, b), 
        MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(RgbColor::BLACK)
            .build(), Alignment::Center)
        .draw(display);
    }
}

//garden example functions

pub fn flower<D>(display: &mut D, n: f64, d: f64, div: f64, mult: f64, count: i32, x_coord: f64, y_coord: f64)
    where 
        D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let (mut a, mut r, mut x, mut y);
    for t in 0..count {
        a = t as f64 * d * (PI as f64 / div);
        r = mult * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + x_coord) as i32, (y + y_coord) as i32), 
            MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(RgbColor::BLACK)
            .build(),  
            Alignment::Center)
        .draw(display);
    }
}

pub fn stem<D>(display: &mut D, sx: i32, sy: i32, ey: i32)
    where 
        D:DrawTarget<Color = Rgb565>+Dimensions {

    Line::new(Point::new(sx, sy), Point::new(sx, ey))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLACK, 2))
        .draw(display);
}



