#![no_std]
#![no_main]

// use spi_dma_displayinterface::spi_dma_displayinterface;
use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::{RgbColor, DrawTarget, Dimensions},
    pixelcolor::Rgb565,
};

use hal::{
    clock::{ClockControl, CpuClock},
    dma::DmaPriority,
    gdma::Gdma,
    peripherals::Peripherals,
    prelude::*,
    spi::{
        master::{prelude::*, Spi}, 
        SpiMode,
    },
    IO,
    Rng,
    Delay,
};

use esp_backtrace as _;

use embedded_graphics_framebuf::FrameBuf;

use examples_assets::{ hat, logo, ferris, tree, gift, gifts, snowflake };

//number of snowflakes per set
const SIZE: usize = 5;
struct Coordinates([u8; SIZE], [i16; SIZE]);
struct SnowflakeSet {
    positions: Coordinates,
    sizes: [u8; SIZE],
}

impl Default for SnowflakeSet {
    fn default() -> SnowflakeSet {
        SnowflakeSet {
            positions: Coordinates([0; SIZE], [0; SIZE]),
            sizes: [0; SIZE],
        }
    }
}

const NUM_SETS: usize = 4;
const STAGGER_OFFSET: i16 = -50;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;
    let cs = io.pins.gpio5;
    let miso = io.pins.gpio2;

    let dc = io.pins.gpio4.into_push_pull_output();
    let mut backlight = io.pins.gpio45.into_push_pull_output();
    let reset = io.pins.gpio48.into_push_pull_output();

    let dma = Gdma::new(peripherals.DMA);
    let dma_channel = dma.channel0;
    
    let mut descriptors = [0u32; 8 * 3];
    let mut rx_descriptors = [0u32; 8 * 3];

    let spi = Spi::new(
        peripherals.SPI2,
        40u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    ).with_pins(
        Some(sclk),
        Some(mosi),
        Some(miso),
        Some(cs),
    );
    // ).with_dma(dma_channel.configure(
    //     false,
    //     &mut descriptors,
    //     &mut rx_descriptors,
    //     DmaPriority::Priority0,
    // ));

    // let di = spi_dma_displayinterface::new_no_cs(320 * 240 * 2, spi, dc);
    let di = SPIInterfaceNoCS::new(spi, dc);
    delay.delay_ms(500u32);
    
    let mut display = match mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(320, 240)
        .with_orientation(mipidsi::Orientation::PortraitInverted(false))
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut delay, Some(reset)) {
        Ok(display) => display,
        Err(_) => {
            panic!("Display initialization failed");
        }
    };

    backlight.set_high().unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    let mut data = [Rgb565::BLACK; 320 * 240];
    let mut fbuf = FrameBuf::new(&mut data, 320, 240);

    let mut snowflake_sets: [SnowflakeSet; NUM_SETS] = Default::default();

    let mut rng = Rng::new(peripherals.RNG);

    //initial random values for all sets
    for (i, set) in snowflake_sets.iter_mut().enumerate() {
        rng.read(&mut set.positions.0).unwrap();
        rng.read(&mut set.sizes).unwrap();
        for y in &mut set.positions.1 {
            *y = STAGGER_OFFSET * (i as i16); // Apply stagger offset based on set index
        }
    }
    
    let mut main_counter: u32 = 0;

    loop {
        hat(&mut fbuf, 64.0, 20.0); 
        logo(&mut fbuf);

        ferris(&mut fbuf);
        hat(&mut fbuf, 166.0, 105.0);

        tree(&mut fbuf);
        gift(&mut fbuf, 250, 215);
        gifts(&mut fbuf, 290, 210);

        for set in snowflake_sets.iter_mut() {
            update_snowflakes(&mut rng, &mut set.positions, &mut set.sizes, SIZE, main_counter, &mut fbuf);
        }
    
        let pixel_iterator = fbuf.into_iter().map(|p| p.1);
        let _ = display.set_pixels(0, 0, 319, 240, pixel_iterator);
    
        #[allow(unused_must_use)] {
            fbuf.clear(Rgb565::BLACK);
        }
    
        // Increment the main counter
        main_counter = main_counter.wrapping_add(5);
    }
}

fn update_snowflakes<D>(rng: &mut Rng, snow_positions: &mut Coordinates, sizes: &mut [u8; SIZE], size: usize, main_counter: u32, fbuf: &mut D) 
where D:DrawTarget<Color = Rgb565>+Dimensions {
    for i in 0..size {
        // Check if the snowflake should start moving or if it's already moving
        if snow_positions.1[i] > 0 || main_counter % (50 * size as u32) >= 50 * i as u32 {
            snow_positions.1[i] += 5; // Move the snowflake down

            // Draw the snowflake
            snowflake(fbuf, snow_positions.0[i] as i32, snow_positions.1[i] as i32, sizes[i] as u32);
        }

        // Reset the snowflake if it reaches the bottom
        if snow_positions.1[i] > 240 {
            snow_positions.1[i] = 0; // Reset to top
            rng.read(&mut snow_positions.0[i..i+1]).unwrap();
            rng.read(&mut sizes[i..i+1]).unwrap();
        }
    }
}