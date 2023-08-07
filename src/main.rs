extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem
        .window("Text Overlay", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create a rendering context
    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    // Set up the text properties
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "/usr/share/fonts/TTF/IBMPlexSansHebrew-Bold.ttf"; // Replace this with the actual path to your TTF font file
    let font_size = 36;
    let font = ttf_context.load_font(font_path, font_size).unwrap();
    let text_color = Color::RGB(255, 255, 255);

    // Create the text surface and texture
    let text_surface = font.render("Hello, Rust with SDL2!").blended(text_color).unwrap();
    let binding = canvas.texture_creator();
    let text_texture = binding.create_texture_from_surface(&text_surface).unwrap();

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Render text overlay
        let texture_query = text_texture.query();
        let x = (800 - texture_query.width) as i32 / 2;
        let y = (600 - texture_query.height) as i32 / 2;

        canvas.copy(&text_texture, None, sdl2::rect::Rect::new(x, y, texture_query.width, texture_query.height)).unwrap();

        // Update the screen
        canvas.present();

        // Add a small delay to control the frame rate
        std::thread::sleep(Duration::from_millis(10));
    }
}
