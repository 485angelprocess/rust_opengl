/************ DRAWING FLAGS *******************/
// Set these to incur things like redrawing vertices etc... //
use std::rc::Rc;
use std::cell::RefCell;
use ogl33::*;
use itertools::Itertools;

// Add as we go
#[derive(PartialEq, Copy, Clone)]
enum DrawFlag{
	REDRAW = 0,
	BROADCAST // set to push these flags up one level
}

// TODO
// Define these size of socket values in preprocess
// so we can do like u8s instead

// ********** SOCKET **********************//
// Special baby
// so cool
// so kind
pub enum SocketOperation{
	SETLATCHED, // will add more
	LATCHTOGGLE
}

struct Socket{ // TODO: probably add name at some point
	latched: i32,
	volatile: i32,
	reference: Rc<RefCell<i32>>, 
	call: Option<fn(i32)>,
	//global_callback: Option<Box<dyn FnMut(&mut State, i32)>>,
	flag: Vec<DrawFlag>,
	max: i32,
	min: i32,
	parent_ref: usize,
	name: String // todo have string identifier //ID info of socket
}

impl Socket{
	fn new(reference: Rc<RefCell<i32>>, call: Option<fn(i32)>, max: i32, min: i32, name: &str) -> Socket{
		// Create a new socket with default values
		Socket{
			latched: 0,
			volatile: 0,
			reference: reference,
			call: call, 
			max: max,
			min: min,
			parent_ref: 0,
			flag: Vec::new(),
			name: String::from(name)
		}
	}
	
	fn set_parent(&mut self, parent: usize){
		self.parent_ref = parent; // for now im just indexing
	}
	
	fn add_flag(&mut self, flag: DrawFlag){
		// Add flag
		self.flag.push(flag);
	}
	
	fn get(&mut self, reset: bool) -> i32{
		/* get the current value */
		let mut v = self.latched + self.volatile;
		
		if v > self.max{
			v = self.max;
		}
		if v <= self.min{
			v = self.min;
		}
		
		if reset{
			self.volatile = 0;
		}
		
		v
	}
	
	fn reset(&mut self){
		/* Reset volatile value */
		self.volatile = 0;
	}
	
	fn hard_reset(&mut self){
		self.latched = 0;
		self.volatile = 0;
	}
	
	fn run(&mut self, flags: &mut Vec<DrawFlag>){
		/* Call when socket is changed */
		let v = self.get(false);
		*self.reference.borrow_mut() = v;
		
		match self.call{
			Some(f) => (f)(v),
			None => ()
		};
		
		for f in &self.flag{
			if ! flags.contains(&f){
				flags.push(*f);
			}
		}
	}

	fn set_latched(&mut self, v: i32){
		self.latched = v;
	}
	
	fn toggle_latched(&mut self){
		if self.latched == self.min{
			self.latched = self.max;
		}
		else{
			self.latched = self.min;
		}
	}
}

/********* Node **********/
pub trait Node{
	// STUB
	fn update(&mut self, _fc: i32, _flags: &Vec<DrawFlag>){}
	fn draw(&self, flags: &Vec<DrawFlag>){
		for f in flags{
			match f{
				DrawFlag::REDRAW => self.redraw(),
				_ => ()
			}
		}
	}
	fn redraw(&self){}
	fn get_sockets(&self) -> Vec<Socket>;
	
}

pub struct Input{
	v: Rc<RefCell<i32>>
}

impl Input{
	fn new() -> Input{
		Input{
			v: Rc::new(RefCell::new(0))
		}
	}
	
	fn get(&self) -> i32{
		*self.v.borrow()
	}
	
	fn get_clone(&self) ->Rc<RefCell<i32>> {
		Rc::clone(&self.v)
	}
}

pub struct GlobalGL{
	r: Input,
	g: Input,
	b: Input,
	a: Input
}

impl GlobalGL{
	pub fn new() -> GlobalGL{
		GlobalGL{
			r: Input::new(),
			g: Input::new(),
			b: Input::new(),
			a: Input::new()
		}
	}
}

impl Node for GlobalGL{
	fn redraw(&self){
		unsafe{
			glClearColor(self.r.get() as f32 / 1000.0, 
				self.g.get() as f32 / 1000.0,
				self.b.get() as f32 / 1000.0,
				self.a.get() as f32 / 1000.0
			);
		}
		//println!("Set r to {}", *self.r.borrow());
	}
	
	fn get_sockets(&self) -> Vec<Socket>{
		let mut v = Vec::new();
		
		let mut s = Socket::new(self.r.get_clone(), None, 1000, 0, "red");
		s.add_flag(DrawFlag::REDRAW);
		v.push(s);
		
		s = Socket::new(self.g.get_clone(), None, 1000, 0, "green");
		s.add_flag(DrawFlag::REDRAW);
		v.push(s);
		
		s = Socket::new(self.g.get_clone(), None, 1000, 0, "blue");
		s.add_flag(DrawFlag::REDRAW);
		v.push(s);
		
		s = Socket::new(self.g.get_clone(), None, 1000, 0, "alpha");
		s.add_flag(DrawFlag::REDRAW);
		v.push(s);
		
		v
	}
}


/********** Main struct ******************/
pub struct GLMain{
	sockets: Vec<Socket>,
	nodes: Vec<Box<dyn Node>>, // may use different structure later, to use parent child topology
	changebuffer: Vec<usize>,
	socketparent: Vec<Vec<*const Socket>>,
	flags: Vec<Vec<DrawFlag>>,
	fc: i32
}

impl GLMain{
	pub fn new() -> GLMain{
		GLMain{
			sockets: Vec::new(),
			nodes: Vec::new(),
			changebuffer: Vec::new(),
			flags: Vec::new(),
			socketparent: Vec::new(),
			fc: 0
		}
	}
	
	pub fn add_node(&mut self, node: Box<dyn Node>){
		// TODO: I think im looking to copy here and not make static lifetime		
		// TODO: here's where we can do some child /parent nonsense
		// tracking
		
		// i think the main thing is a redraw flag to a parent should cascade down
		// to the children
		self.nodes.push(node);
		self.socketparent.push(Vec::new());
		for mut socket in self.nodes[self.nodes.len() - 1].get_sockets(){
			socket.set_parent(self.nodes.len() - 1); // save who our parent is
			self.sockets.push(socket); // accumulate all of our sockets
			
			self.socketparent[self.nodes.len() - 1].push(&self.sockets[self.sockets.len() - 1]); // save an index of which socket goes to which parent (USE POINTER/REF INSTEAD?)
		}
		self.flags.push(Vec::new());
	}
	
	pub fn modify(&mut self, socket_index: usize, operation: SocketOperation, v: i32){
		// Modify a socket
		match operation{
			SocketOperation::SETLATCHED => self.sockets[socket_index].set_latched(v),
			SocketOperation::LATCHTOGGLE => self.sockets[socket_index].toggle_latched(),
			_ => ()
		};
		
		//println!("Socket {} modified", socket_index);
		self.changebuffer.push(socket_index);
	}
	
	pub fn unwind(&mut self){
		/*between frames change all our little values*/		
		for i in self.changebuffer.iter().unique(){
			//println!("Socket {} changed", i);
			let s = &mut self.sockets[*i];
			//let node = &self.nodes[s.parent_ref];
			s.run(&mut self.flags[s.parent_ref]);
		}
	}
	
	pub fn draw(&mut self){
		self.unwind();
		
		for i in 0..self.nodes.len(){
			
			self.nodes[i].update(self.fc, &self.flags[i]);
			self.nodes[i].draw(&self.flags[i]);
				// TODO special operations etc...
			self.flags[i] = Vec::new();
		}
		self.fc += 1;
	}
	
	pub fn print_structure(&self){
		for i in 0..self.nodes.len(){
			println!("Node {}", i);
			for s in &self.socketparent[i]{
				unsafe{
					println!("\t-> {}", (*(*s)).name);
				}
			}
		}
	}
}