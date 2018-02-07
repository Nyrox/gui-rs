use {Context, Color, Rect, Side, DeclaredSize, Widget};
use style::*;

use util::UnsafePtr;

#[derive(Debug, Default)]
pub struct ComputedStyle {
	pub border_box: Rect<f64>,
	pub content_box: Rect<f64>
}

#[derive(Debug)]
pub struct Element {
	pub children: Vec<Element>,
	pub computed_style: ComputedStyle,
	pub id: u64,
	pub context: UnsafePtr<Context>,
	pub parent: UnsafePtr<Element>,
	pub style: DeclaredStyle
}

impl Element {
	pub fn new(context: &mut Context, style: DeclaredStyle) -> Element {		
		Element {
			computed_style: ComputedStyle::default(),
			children: Vec::new(),
			id: context.generate_element_id(),
			context: UnsafePtr::from(context),
			parent: UnsafePtr::null(),
			style
		}
	}
	
	/*
	Adds a child to the element.
	*/
	pub fn add_child(&mut self, mut element: Element) -> Result<(), ()> {
		element.parent = UnsafePtr::from(&mut (*self));
		self.children.push(element);
		return Ok(());
	}

	pub fn reflow(&mut self) {
		let line_width = match self.style.width {
			DeclaredSize::Auto => {
				match self.parent {
					_ if self.parent.is_null() => { (*self.context).width },
					_ => { (*self.parent).computed_style.content_box.width }
				}
			},
			DeclaredSize::Pixels(p) => {
				if self.parent.is_null() {
					p
				} else {
					self.computed_style.border_box.width
				}
			}
			_ => { panic!() }
		};

		self.computed_style.border_box.width = line_width;
		self.computed_style.border_box.height = 100.0;
		self.computed_style.content_box = self.computed_style.border_box;
		
		self.computed_style.content_box.left += self.style.padding.left.unwrap_as_pixels();
		self.computed_style.content_box.top += self.style.padding.top.unwrap_as_pixels();
		self.computed_style.content_box.width -= self.style.padding.left.unwrap_as_pixels() + self.style.padding.right.unwrap_as_pixels();
		self.computed_style.content_box.height -= self.style.padding.top.unwrap_as_pixels() + self.style.padding.bottom.unwrap_as_pixels();
		
		let line_width = self.computed_style.content_box.width;
		
		let mut current_offset = 0.0;
		for child in &mut self.children {
			// Le algorithm
			let width = match child.style.width {
				DeclaredSize::Pixels(p) => p,
				_ => panic!()
			};
			child.computed_style.border_box.width = width;
			child.computed_style.border_box.left = self.computed_style.content_box.left + current_offset;
			child.computed_style.border_box.top = self.computed_style.content_box.top;
			
			child.computed_style.content_box = child.computed_style.border_box;
			
			current_offset += width;
			
			child.reflow();
		}
	}
}