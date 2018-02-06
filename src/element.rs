use {Context, Color, Rect, Side, DeclaredSize, Widget};


#[derive(Debug, Default)]
pub struct ComputedStyle {
	pub border_box: Rect<f64>,
	pub content_box: Rect<f64>
}

#[derive(Debug)]
pub struct Element {
	pub widget: Box<Widget>,
	pub children: Vec<Element>,
	pub computed_style: ComputedStyle,
	pub id: u64,
	pub context: *mut Context,
	pub parent: *mut Element,
	pub color: Color<f32>,
	pub padding_color: Color<f32>
}

impl Element {
	pub fn new<W: 'static + Widget>(context: &mut Context, w: W) -> Element {
		
		Element {
			widget: Box::new(w),
			computed_style: ComputedStyle::default(),
			children: Vec::new(),
			id: context.generate_element_id(),
			context: context as *mut Context,
			parent: ::std::ptr::null_mut(),
			color: Color::default(),
			padding_color: Color::default()
		}
	}
	
	/*
	Adds a child to the element.

	Errors if the underlying widget is not a container.
	*/
	pub fn add_child(&mut self, mut element: Element) -> Result<(), ()> {
		if !self.widget.is_container() { return Err(()) };

		element.parent = self as *mut Element;
		self.children.push(element);
		return Ok(());
	}

	pub fn reflow(&mut self) {
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
		
		println!("{:?}", unsafe { (*self.context).width });
		println!("{:?}", line_width);
		self.computed_style.border_box.width = line_width;
		self.computed_style.border_box.height = 100.0;
		self.computed_style.content_box = self.computed_style.border_box;
		
		self.computed_style.content_box.left += self.widget.get_padding(Side::Left).unwrap_as_pixels();
		self.computed_style.content_box.top += self.widget.get_padding(Side::Top).unwrap_as_pixels();
		self.computed_style.content_box.width -= self.widget.get_padding(Side::Left).unwrap_as_pixels() + self.widget.get_padding(Side::Right).unwrap_as_pixels();
		self.computed_style.content_box.height -= self.widget.get_padding(Side::Top).unwrap_as_pixels() + self.widget.get_padding(Side::Bottom).unwrap_as_pixels();
		
		let line_width = self.computed_style.content_box.width;
		
		let mut current_offset = 0.0;
		for child in &mut self.children {
			// Le algorithm
			let width = match child.widget.get_declared_width(){
				DeclaredSize::Pixels(p) => p,
				_ => panic!()
			};
			child.computed_style.border_box.width = width;
			child.computed_style.border_box.left = current_offset;
			child.computed_style.border_box.top = self.computed_style.content_box.top;
			
			child.computed_style.content_box = child.computed_style.border_box;
			
			current_offset += width;
			
			child.reflow();
		}
	}
}