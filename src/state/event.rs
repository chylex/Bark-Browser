use std::cell::RefCell;
use std::rc::Rc;

use crate::state::Environment;

pub trait Event<L> {
	fn dispatch(&self, layer: &mut L, environment: &Environment) -> EventResult;
}

impl<L, F> Event<L> for F where F: Fn(& mut L, &Environment) -> EventResult {
	fn dispatch(&self, layer: &mut L, environment: &Environment) -> EventResult {
		self(layer, environment)
	}
}

pub struct EventQueue<L> {
	events: Rc<RefCell<Vec<Box<dyn Event<L>>>>>
}

impl<L> EventQueue<L> {
	pub fn new() -> Self {
		Self { events: Rc::new(RefCell::new(Vec::new())) }
	}
	
	pub fn rc_clone(&self) -> Self {
		Self { events: Rc::clone(&self.events) }
	}
	
	pub fn enqueue<E: Event<L> + 'static>(&self, event: E) -> bool {
		if let Ok(mut events) = self.events.try_borrow_mut() {
			events.push(Box::new(event));
			true
		} else {
			false
		}
	}
	
	pub fn enqueue_fn<F>(&self, event: F) -> bool where F: Fn(&mut L, &Environment) -> EventResult + 'static {
		self.enqueue(event)
	}
	
	pub fn take(&self) -> Vec<Box<dyn Event<L>>> {
		self.events.take()
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EventResult {
	Nothing,
	Draw,
	Redraw,
}

impl EventResult {
	pub const fn draw_if(condition: bool) -> Self {
		if condition {
			Self::Draw
		} else {
			Self::Nothing
		}
	}
	
	pub fn merge(self, other: Self) -> Self {
		if self == Self::Redraw || other == Self::Redraw {
			Self::Redraw
		} else if self == Self::Draw || other == Self::Draw {
			Self::Draw
		} else {
			Self::Nothing
		}
	}
}
