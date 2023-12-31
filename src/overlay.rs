#[cfg(feature = "overlay")]
pub mod overlay {
    extern crate sdl2;
    use crossbeam::channel::Receiver;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    pub(crate) fn render_ovrelay(rec: Arc<Mutex<Receiver<String>>>) {
        // Initialize SDL2
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Create a window
        let window = video_subsystem
            .window("Text Overlay", 200, 100)
            .build()
            .unwrap();

        // Create a rendering context
        let mut canvas = window.into_canvas().build().unwrap();

        // Set up the text properties
        let ttf_context = sdl2::ttf::init().unwrap();
        let font_path = "/usr/share/fonts/TTF/IBMPlexSansHebrew-Bold.ttf";
        // Replace this with the actual path to your TTF font file
        let font_size = 12;
        let font = ttf_context.load_font(font_path, font_size).unwrap();
        let text_color = Color::RGB(255, 255, 255);

        // Main loop
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut current_layer = String::from("main"); // TODO: Optimize this, global app wide, lazy-static
                                                      // variable for this?
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

            // Create the text surface and texture
            // generate a random string

            let layer_changed = rec.lock().unwrap().recv();

            // Returns ok when there is a layer change
            // Error if nothing changed
            if let Ok(new_layer) = layer_changed {
                current_layer = new_layer
            }

            let text_surface = font.render(&current_layer).blended(text_color).unwrap();
            let binding = canvas.texture_creator();
            let text_texture = binding.create_texture_from_surface(&text_surface).unwrap();

            // Clear the canvas
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // Render text overlay
            let texture_query = text_texture.query();
            let x = 0;
            let y = 0;

            canvas
                .copy(
                    &text_texture,
                    None,
                    sdl2::rect::Rect::new(x, y, texture_query.width, texture_query.height),
                )
                .unwrap();

            // Update the screen
            canvas.present();

            // Add a small delay to control the frame rate
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}
