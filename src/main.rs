extern crate piston_window;
use piston_window::*;

struct Color<T> {
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
struct Rect<T> {
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
enum Direction {
	Horizontal,
	Vertical
}

impl Default for Direction {
	fn default() -> Direction {
		Direction::Horizontal
	}
}

#[derive(Clone, Copy, Debug)]
enum DeclaredSize {
	Auto,
	Pixels(f32),
	Percent(f32)
}

impl Default for DeclaredSize {
	fn default() -> DeclaredSize {
		DeclaredSize::Auto
	}
}

trait Widget : ::std::fmt::Debug {
	fn is_container(&self) -> bool {
		false
	}
	fn get_direction(&self) -> Direction {
		Direction::Horizontal
	}
	fn get_declared_size(&self) -> DeclaredSize {
		DeclaredSize::Auto
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

impl Widget for Button {}

#[derive(Default, Clone, Copy, Debug)]
struct Text {

}


fn flex<T: Widget>(widget: &T) -> Rect<f64> {


	Rect::new(50.0, 100.0, 100.0, 100.0)
}

macro_rules! draw_rect {
	($rect:expr, $color:expr, $c:expr, $g:expr) => {{
		rectangle([$color.r, $color.g, $color.b, $color.a], [$rect.left, $rect.top, $rect.width, $rect.height], $c.transform, $g);
	}}
}

#[derive(Debug, Default)]
struct ComputedStyle {
	border_box: Rect<f64>,
	content_box: Rect<f64>
}

#[derive(Debug)]
struct Element {
	widget: Box<Widget>,
	children: Vec<Element>,
	computed_style: ComputedStyle,
	id: u64,
	context: *mut Context,
	parent: *mut Element
}

impl Element {
	/*
	Adds a child to the element.

	Errors if the underlying widget is not a container.
	*/
	fn add_child(&mut self, mut element: Element) -> Result<(), ()> {
		if !self.widget.is_container() { return Err(()) };

		element.parent = self as *mut Element;
		self.children.push(element);
		return Ok(());
	}

	fn reflow(&mut self) {
		let declared_size = self.widget.get_declared_size();
		let line_width = match declared_size {
			DeclaredSize::Auto => {
				let nullptr = 0 as *mut Element;
				unsafe {
					match self.parent {
						nullptr => { (*self.context).width },
						_ => { (*self.parent).computed_style.content_box.width }
					}
				}
			},
			_ => { panic!() }
		};

		println!("{:?}", line_width);
		self.computed_style.border_box = Rect::new(50.0, 100.0, 100.0, 100.0);
	}
}

#[derive(Debug, Default)]
struct Context {
	counter: u64,
	width: f64
}

impl Context {
	fn create_element<W: Widget + 'static>(&mut self, widget: W) -> Element {
		self.counter += 1;
		Element {
			context: self as *mut Context,
			widget: Box::new(widget),
			id: self.counter,
			children: Vec::new(),
			computed_style: ComputedStyle::default(),
			parent: 0 as *mut Element
		}
	}
}

fn main() {
	let mut context = Context::default();
	context.width = 1280.0;

	let mut container = Container::default();
	let mut container = context.create_element(container);

	container.add_child(context.create_element(Button {
		width: DeclaredSize::Pixels(128.0),
		height: DeclaredSize::Pixels(48.0)
	})).unwrap();

	container.add_child(context.create_element(Button {
		width: DeclaredSize::Pixels(128.0),
		height: DeclaredSize::Pixels(48.0)
	})).unwrap();


	container.reflow();
	println!("{:?}", container);

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
		window.draw_2d(&e, |c, g| {

			clear([0.0, 0.0, 0.0, 1.0], g);
			draw_rect!(container.computed_style.border_box, Color::<f32>::new(1.0, 0.0, 0.0, 1.0), c, g);
		});
	}
}
