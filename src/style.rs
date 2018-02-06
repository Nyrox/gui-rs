
use DeclaredSize;
use Color;

#[derive(Debug, Default)]
pub struct SidesArray<T> {
	pub top: T,
	pub right: T,
	pub bottom: T,
	pub left: T
}

#[derive(Debug, Default)]
pub struct DeclaredStyle {
	pub width: DeclaredSize,
	pub height: DeclaredSize,
	pub background_color: Color<f32>,
	pub padding_color: Color<f32>,
	pub padding: SidesArray<DeclaredSize>
}

impl DeclaredStyle {
	
}