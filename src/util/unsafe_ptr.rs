use std::ptr;
use std::ops::{Deref, DerefMut};

/*
Provides an internally unsafe pointer.
By using this, you are signalling that you are making yourself responsible for anything that happens.
For those moments where you dont wanna write C++, but Rust is being annoying.
*/

#[derive(Debug, Clone, Copy)]
pub struct UnsafePtr<T> {
	_data: *mut T
}

impl<T> UnsafePtr<T> {
	pub fn null() -> Self {
		UnsafePtr { _data: ptr::null_mut() }
	}
	
	pub fn is_null(&self) -> bool {
		self._data.is_null()
	}
}

/*
Deref Const
*/
impl<T> Deref for UnsafePtr<T> {
	type Target = T;
	
	fn deref(&self) -> &T {
		unsafe {
			& (*self._data)	
		}
	}
}

/*
Deref mut
*/
impl<T> DerefMut for UnsafePtr<T> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe {
			&mut (*self._data)
		}
	}
}

/*
From mut ref
*/
impl<'a, T> From<&'a mut T> for UnsafePtr<T> {
	fn from(_ref: &mut T) -> Self {
		UnsafePtr {
			_data: _ref as *mut T
		}
	}
}

/*
From mut ptr
*/
impl<T> From<*mut T> for UnsafePtr<T> {
	fn from(ptr: *mut T) -> Self {
		UnsafePtr {
			_data: ptr
		}
	}
}