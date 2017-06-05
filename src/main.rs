extern crate sdl2;
extern crate svg_now;
extern crate svg;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn render(canvas: &mut Canvas<Window>, render_text: bool) {
    let ttf_context = sdl2::ttf::init().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, 256, 256).unwrap();
    let font = ttf_context.load_font("\\Windows\\Fonts\\simfang.ttf", 128).unwrap();
    const DATA: &'static [&'static str] = &[
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
           <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="15" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="15" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="15" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="30" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="15" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="15" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="0" y1="15" x2="15" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>"#,
            r#"
            <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <line x1="15" y1="15" x2="30" y2="15" stroke-width="2" stroke="black"/>
		        <line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/>
	        </svg>
            "#,
            r#"<svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		        <text x="15" y="15" font-family="Verdana" text-anchor="middle" alignment-baseline="middle" font-size="15">&#x1f47b;</text>
	        </svg>"#,
    ];
    for (idx, src) in DATA.iter().enumerate() {
        let svg_content = svg::parse(src).unwrap();

        let svg_rendered = svg_now::render((256, 256), svg_content);
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y*pitch + x*3;
                    assert_eq!(pitch, 256 * 3);
                    buffer[offset] = svg_rendered[(x + y * 256) * 4];
                    buffer[offset + 1] = svg_rendered[(x + y * 256) * 4 + 1];
                    buffer[offset + 2] = svg_rendered[(x + y * 256) * 4 + 2];
                }
            }
        }).unwrap();
        
        canvas.copy(&texture, None, Some(Rect::new(10 + idx as i32 / 3 * 200, 10 + idx as i32 % 3 * 200, 180, 180))).unwrap();
        
        if render_text {
            let surface = font.render(&format!("{}", idx)).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

            canvas.copy(&texture, None, Some(Rect::new(10 + idx as i32 / 3 * 200, 10 + idx as i32 % 3 * 200, 180, 180))).unwrap();
        }

    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    

    canvas.clear();

    render(&mut canvas, true);
    
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut render_text = true;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown {..} => {
                    render_text = !render_text;
                    canvas.clear();
                    render(&mut canvas, render_text);
                    canvas.present();
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}
