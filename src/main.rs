#![recursion_limit="128"]

extern crate piston_window;
use piston_window::*;

#[derive(Default, Clone, Copy, Debug)]
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
	Pixels(f64),
	Percent(f64)
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
	fn get_declared_width(&self) -> DeclaredSize {
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
	draw_rect(&mut elem.computed_style.border_box, elem.color, c, g);
	for mut child in &mut elem.children {
		render(child, c, g);
	}
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
	parent: *mut Element,
	color: Color<f32>
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
		let declared_width = self.widget.get_declared_width();
		let line_width = match declared_width {
			DeclaredSize::Auto => {
				unsafe {
					match self.parent {
						_ if self.parent.is_null() => { (*self.context).width },
						_ => { (*self.parent).computed_style.content_box.width }
					}
				}
			},
			DeclaredSize::Pixels(p) => {
				unsafe {
					if self.parent.is_null() {
						p
					} else {
						self.computed_style.border_box.width
					}
				}
			}
			_ => { panic!() }
		};

		println!("{:?}", line_width);
		self.computed_style.border_box.width = line_width;
		self.computed_style.border_box.height = 100.0;
		
		let mut current_offset = 0.0;
		for child in &mut self.children {
			// Le algorithm
			let width = match child.widget.get_declared_width(){
				DeclaredSize::Pixels(p) => p,
				_ => panic!()
			};
			child.computed_style.border_box.width = width;
			child.computed_style.border_box.left = current_offset;
			
			current_offset += width;
			
			child.reflow();
		}
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
			parent: 0 as *mut Element,
			color: Color::new(1.0, 0.0, 0.0, 1.0)
		}
	}
}

fn main() {
	let mut context = Context::default();
	context.width = 1280.0;

	let mut container = Container::default();
	let mut container = context.create_element(container);
	container.color = Color::new(0.0, 1.0, 0.0, 1.0);
	
	container.add_child(context.create_element(Button {
		width: DeclaredSize::Pixels(128.0),
		height: DeclaredSize::Pixels(48.0)
	})).unwrap();
	container.children[0].color = Color::new(0.0, 0.0, 1.0, 1.0);
	
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
		window.draw_2d(&e, |c, mut g| {

			clear([0.0, 0.0, 0.0, 1.0], g);
			render(&mut container, c, &mut g);
		});
	}
}
