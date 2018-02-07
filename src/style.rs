/* 4D Array */
#[derive(Debug, Default)]
pub struct SidesArray<T> {
	pub top: T,
	pub right: T,
	pub bottom: T,
	pub left: T
}

/* Color */
#[derive(Default, Clone, Copy, Debug)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
		Color { r, g, b, a }
	}
}

/*
Declared Size
*/
#[derive(Clone, Copy, Debug)]
pub enum DeclaredSize {
	Auto,
	Pixels(f64),
	Percent(f64)
}

impl DeclaredSize {
	pub fn unwrap_as_pixels(&self) -> f64 {
		match *self {
			DeclaredSize::Pixels(p) => p,
			DeclaredSize::Auto => 16.0,
			_ => panic!()
		}
	}
}

/* DeclaredSize Default */
impl Default for DeclaredSize {
	fn default() -> DeclaredSize {
		DeclaredSize::Auto
	}
}

/* Declared Style */
#[derive(Debug, Default)]
pub struct DeclaredStyle {
	pub width: DeclaredSize,
	pub height: DeclaredSize,
	pub background_color: Color,
	pub padding_color: Color,
	pub padding: SidesArray<DeclaredSize>
}

impl DeclaredStyle {
	
}