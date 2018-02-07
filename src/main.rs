extern crate piston_window;
use piston_window::*;

mod element;
use element::{Element, ComputedStyle};

mod style;
use style::*;

mod util;
use util::UnsafePtr;

#[derive(Default, Clone, Copy, Debug)]
pub struct Rect<T> {
	left: T,
	top: T,
	width: T,
	height: T
}

impl<T> Rect<T> {
	pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
		Rect { left, top, width, height }
	}
}

#[derive(Clone, Copy, Debug)]
pub enum Side {
	Top,
	Right,
	Bottom,
	Left
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
	Horizontal,
	Vertical
}

impl Default for Direction {
	fn default() -> Direction {
		Direction::Horizontal
	}
}

fn draw_rect(rect: &mut Rect<f64>, color: Color, c: piston_window::Context, g: &mut G2d) {
	rectangle([color.r, color.g, color.b, color.a], [rect.left, rect.top, rect.width, rect.height], c.transform, g);
}

fn render(elem: &mut Element, c: piston_window::Context, g: &mut G2d) {
	draw_rect(&mut elem.computed_style.border_box, elem.style.padding_color, c, g);
	draw_rect(&mut elem.computed_style.content_box, elem.style.background_color, c, g);
	for mut child in &mut elem.children {
		render(child, c, g);
	}
}

#[derive(Debug)]
pub struct Context {
	counter: u64,
	width: f64,
	height: f64,
	root: Element
}

impl Context {
	fn new(width: f64, height: f64) -> Context {
		Context { counter: 0, width, height, root: Element {
			context: UnsafePtr::null(),
			id: 0,
			children: Vec::new(),
			computed_style: ComputedStyle::default(),
			parent: UnsafePtr::null(),
			style: DeclaredStyle::default()
		}}
	}
	
	fn generate_element_id(&mut self) -> u64 {
		self.counter += 1;
		return self.counter;
	}
}

fn main() {
	let mut context = Context::new(1280.0, 720.0);
	context.root.context = UnsafePtr::from(&mut context);
	context.root.style.background_color = Color::new(0.1, 0.1, 0.1, 1.0);
	context.root.style.padding_color = Color::new(0.05, 0.05, 0.05, 1.0);
	
	let button = Element::new(&mut context, DeclaredStyle {
		width: DeclaredSize::Pixels(128.0),
		height: DeclaredSize::Pixels(96.0),
		background_color: Color::new(9.0, 0.0, 1.0, 1.0),
		padding_color: Color::new(0.0, 0.0, 0.6, 1.0),
		..DeclaredStyle::default()
	});	
	context.root.add_child(button).unwrap();
	

	let button = Element::new(&mut context, DeclaredStyle {
		width: DeclaredSize::Pixels(128.0),
		height: DeclaredSize::Pixels(96.0),
		background_color: Color::new(1.0, 0.0, 0.0, 1.0),
		padding_color: Color::new(0.6, 0.0, 0.0, 1.0),
		..DeclaredStyle::default()
	});
	context.root.add_child(button).unwrap();

	context.root.reflow();
	println!("{:?}", (*context.root.context).width);

	let mut window: PistonWindow = WindowSettings::new(
		"piston: hello_world",
		[1280, 720]
	)
	.exit_on_esc(true)
	.build()
	.unwrap();


	// println!("{:?}", container);

	window.set_lazy(true);
	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, mut g| {

			clear([0.0, 0.0, 0.0, 1.0], g);
			render(&mut context.root, c, &mut g);
		});
	}
}
