extern crate piston_window;
use piston_window::*;

mod element;
use element::{Element, ComputedStyle};

#[derive(Default, Clone, Copy, Debug)]
pub struct Color<T> {
	r: T,
	g: T,
	b: T,
	a: T
}

impl<T> Color<T> {
	pub fn new(r: T, g: T, b: T, a: T) -> Color<T> {
		Color { r, g, b, a }
	}
}

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

#[derive(Clone, Copy, Debug)]
pub enum DeclaredSize {
	Auto,
	Pixels(f64),
	Percent(f64)
}

impl Default for DeclaredSize {
	fn default() -> DeclaredSize {
		DeclaredSize::Auto
	}
}

impl DeclaredSize {
	fn unwrap_as_pixels(&self) -> f64 {
		match *self {
			DeclaredSize::Pixels(p) => p,
			_ => panic!()
		}
	}
}

pub trait Widget : ::std::fmt::Debug {
	fn is_container(&self) -> bool {
		false
	}
	fn get_direction(&self) -> Direction {
		Direction::Horizontal
	}
	fn get_declared_width(&self) -> DeclaredSize {
		DeclaredSize::Auto
	}
	fn get_padding(&self, side: Side) -> DeclaredSize {
		DeclaredSize::Pixels(8.0)
	}
}

#[derive(Default, Debug)]
struct Container {

}

impl Widget for Container {
	fn is_container(&self) -> bool {
		true
	}
}

#[derive(Default, Clone, Copy, Debug)]
struct Button {
	width: DeclaredSize,
	height: DeclaredSize
}

impl Widget for Button {
	fn get_declared_width(&self) -> DeclaredSize {
		self.width
	}
}

#[derive(Default, Clone, Copy, Debug)]
struct Text {

}

fn draw_rect(rect: &mut Rect<f64>, color: Color<f32>, c: piston_window::Context, g: &mut G2d) {
	rectangle([color.r, color.g, color.b, color.a], [rect.left, rect.top, rect.width, rect.height], c.transform, g);
}

fn render(elem: &mut Element, c: piston_window::Context, g: &mut G2d) {
	draw_rect(&mut elem.computed_style.border_box, elem.padding_color, c, g);
	draw_rect(&mut elem.computed_style.content_box, elem.color, c, g);
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
			context: 0 as *mut Context,
			widget: Box::new(Container::default()),
			id: 0,
			children: Vec::new(),
			computed_style: ComputedStyle::default(),
			parent: 0 as *mut Element,
			color: Color::new(1.0, 0.0, 0.0, 1.0),
			padding_color: Color::new(1.0, 0.0, 0.0, 1.0)
		}}
	}
	
	fn create_element<W: Widget + 'static>(&mut self, widget: W) -> Element {
		self.counter += 1;
		Element {
			context: self as *mut Context,
			widget: Box::new(widget),
			id: self.counter,
			children: Vec::new(),
			computed_style: ComputedStyle::default(),
			parent: 0 as *mut Element,
			color: Color::new(1.0, 0.0, 0.0, 1.0),
			padding_color: Color::new(1.0, 0.0, 0.0, 1.0)
		}
	}
	
	fn generate_element_id(&mut self) -> u64 {
		self.counter += 1;
		return self.counter;
	}
}

fn main() {
	let mut context = Context::new(1280.0, 720.0);
	context.root.context = &mut context as *mut _;
	context.root.color = Color::new(0.1, 0.1, 0.1, 1.0);
	context.root.padding_color = Color::new(0.05, 0.05, 0.05, 1.0);
	
	let button = Element::new(&mut context, Button {
		width: DeclaredSize::Pixels(128.0),
		height: DeclaredSize::Pixels(48.0)
	});
	
	context.root.add_child(button).unwrap();
	context.root.children[0].color = Color::new(0.0, 0.0, 1.0, 1.0);
	context.root.children[0].padding_color = Color::new(0.0, 0.0, 0.6, 1.0);
	
	let button = Element::new(&mut context, Button {
		width: DeclaredSize::Pixels(128.0),
		height: DeclaredSize::Pixels(48.0)
	});
	context.root.add_child(button).unwrap();
	context.root.children[1].color = Color::new(1.0, 0.0, 0.0, 1.0);
	context.root.children[1].padding_color = Color::new(0.6, 0.0, 0.0, 1.0);

	context.root.reflow();
	println!("{:?}", unsafe { (*context.root.context).width });

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
