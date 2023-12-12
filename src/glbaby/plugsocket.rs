use std::sync::Arc;

pub struct Socket{
	latched: i32,
	volatile: i32,
	min: i32,
	max: i32,
	scale: i32
}

impl Socket{	
	fn get(&mut self, reset: bool) -> i32{
		/* Get the current value in the socket */
		let mut v = self.latched + self.volatile;
		
		if v > self.max{
			v = self.max;
		}
		if v <= self.min{
			v = self.min;
		}
		
		// TODO: add scaling
		// TODO: may need explicit clear of volatile
		if reset{
			self.clear();
		}
		
		v
	}
	
	fn clear(&mut self){
		self.volatile = 0
	}
	
	fn set_latched(&mut self, v: i32){
		/* Set the value, doesn't reset until changed */
		self.latched = v;
	}
	
	fn add_volatile(&mut self, v: i32){
		/* Add a value that resets every frame */
		self.volatile += v;
	}
}

pub enum Operation{
	SETLATCH,
	ADDVOLATILE
}

pub struct Plug{
	socket: Option<Box<Socket>>,
	operation: Operation,
	priority: u8,
	value: i32
}

impl Plug{
	fn isActive(&self) -> bool{
		match self.socket {
			Some(p) => true,
			None => false
		}
	}
	
	fn set(&mut self, v:i32){
		/* Set the value of the socket */
		self.value = v;
	}
	
	fn write(&self){
		/* Write this value to corresponding socket */
		// TODO: make the sockets a container so oen plug can go to multiple sockets
		if self.socket.is_some(){
			let mut s = self.socket.unwrap();
			match self.operation{
				Operation::SETLATCH => s.set_latched(self.value),
				Operation::ADDVOLATILE => s.add_volatile(self.value)
			};
		}
	}
}