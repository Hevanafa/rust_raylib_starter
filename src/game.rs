use std::io;
use std::io::Write;
use raylib::prelude::*;

const WW: i32 = 640;
const WH: i32 = 360;

pub fn main() {
	println!("Starting raylib-rs...");
	io::stdout().flush().expect("Unable to flush!");

	let (mut rl, thread) = raylib::init().build();

	rl.set_window_size(WW, WH);

	let cornflower_blue = Color::new(0x64, 0x95, 0xed, 0xff);
	let raylib_logo = rl
		.load_texture(&thread, "assets/images/raylib-rust_256x256.png")
		.expect("Unable to load raylib-rust_256x256.png!");

	let raylib_icon = Image::load_image("assets/images/window_icon.png")
		.expect("Unable to load window_icon.png!");
	rl.set_window_icon(&raylib_icon);
	rl.set_window_title(&thread, "Raylib + Rust Starter");
	
	rl.set_target_fps(60);

	let mut t: i32 = 0;

	while !rl.window_should_close() {
		// update
		t += 1;

		// draw
		let mut d = rl.begin_drawing(&thread);

		d.clear_background(cornflower_blue);
		d.draw_text("Hello Raylib + Rust!", 20, 20, 32, Color::RAYWHITE);
		// d.draw_texture(&raylib_logo, 20, 60, Color::RAYWHITE);

		{
			let mut d = d.begin_blend_mode(BlendMode::BLEND_ALPHA);
			let src_rec = Rectangle::new(
				0.0, 0.0,
				raylib_logo.width as f32, raylib_logo.height as f32);

			let dest_rec = Rectangle::new(
				WW as f32 / 2.0, WH as f32 / 2.0,
				raylib_logo.width as f32, raylib_logo.height as f32);

			d.draw_texture_pro(
				&raylib_logo,
				src_rec, dest_rec,
				Vector2::new(
					raylib_logo.width as f32 / 2.0,
					raylib_logo.height as f32 / 2.0),
				0.0,
				Color::RAYWHITE.alpha((1.0 + (t / 30 % 2) as f32) / 2.0));
		}
	}
}