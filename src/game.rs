use raylib::prelude::*;

const WW: i32 = 640;
const WH: i32 = 360;

pub fn main() {
	let (mut rl, thread) = raylib::init().build();

	print!("Hello");

	rl.set_window_size(WW, WH);

	let cornflower_blue = Color::new(0x64, 0x95, 0xed, 0xff);
	let raylib_logo = rl
		.load_texture(&thread, "assets/images/raylib-rust_256x256.png")
		.expect("Unable to load raylib-rust_256x256.png!");

	rl.set_window_title(&thread, "Raylib + Rust Starter");
	rl.set_target_fps(60);

	// Todo: draw the texture with alpha

	let mut t: i32 = 0;

	while !rl.window_should_close() {
		// update
		t += 1;

		// draw
		let mut d = rl.begin_drawing(&thread);
		d.clear_background(cornflower_blue);
		d.draw_text("Hello Raylib + Rust!", 20, 20, 32, Color::RAYWHITE);
		d.draw_texture(&raylib_logo, 20, 60, Color::RAYWHITE);
	}
}