#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]
use beryllium::*;
use core::{
	convert::{TryFrom, TryInto},
	mem::{size_of, size_of_val}
};
use ogl33::*;

mod glbaby;

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = 
	[[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
	
const VERT_SHADER: &str = r#"#version 330 core
	layout (location = 0) in vec3 pos;
	
	void main(){
		gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
	}
"#;

const FRAG_SHADER: &str = r#"#version 330 core
	out vec4 final_color;
	
	void main(){
		final_color = vec4(1.0, 0.5, 0.2, 1.0);
	}
"#;

const WINDOW_TITLE: &str = "window name";

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("Couldn't start SDL");
  
  //Set version context
  sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 4).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 4).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
  
  // create window with options
  let win = sdl.create_gl_window(
        WINDOW_TITLE,
		WindowPosition::Centered,
        800,
        600,
        WindowFlags::Shown,
  ).expect("Couldn't make a window and context");
  win.set_swap_interval(SwapInterval::Vsync);
  
  unsafe {
	load_gl_with(|f_name| win.get_proc_address(f_name));
	glClearColor(0.2, 0.3, 0.3, 1.0);
	
	// create and bind vertex arrays
	let mut vao = 0;
	glGenVertexArrays(1, &mut vao);
	assert_ne!(vao, 0);
	glBindVertexArray(vao);
	
	let mut vbo = 0;
	glGenBuffers(1, &mut vbo);
	assert_ne!(vbo, 0);
	glBindBuffer(GL_ARRAY_BUFFER, vbo);
	glBufferData(
		GL_ARRAY_BUFFER,
		size_of_val(&VERTICES) as isize,
		VERTICES.as_ptr().cast(),
		GL_STATIC_DRAW,
	);
	
	glVertexAttribPointer(
		0,
		3,
		GL_FLOAT,
		GL_FALSE,
		size_of::<Vertex>().try_into().unwrap(),
		0 as *const _,
	);
	
	glEnableVertexAttribArray(0);
	
	// load an compile vertex shader
	let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
	assert_ne!(vertex_shader, 0);
	glShaderSource(
		vertex_shader,
		1,
		&(VERT_SHADER.as_bytes().as_ptr().cast()),
		&(VERT_SHADER.len().try_into().unwrap())
	);
	
	glCompileShader(vertex_shader);
	let mut success = 0;
	glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
	if success == 0 {
		// shader error
		let mut v: Vec<u8> = Vec::with_capacity(1024);
		let mut log_len = 0_i32; // number of bytes in log
		glGetShaderInfoLog(
			vertex_shader,
			1024,
			&mut log_len,
			v.as_mut_ptr().cast(),
		);
		v.set_len(log_len.try_into().unwrap());
		panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
	}
	
	// load fragment shader
	let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
	assert_ne!(fragment_shader, 0);
	glShaderSource(
		fragment_shader,
		1,
		&(FRAG_SHADER.as_bytes().as_ptr().cast()),
		&(FRAG_SHADER.len().try_into().unwrap()),
	);
	
	// compile fragment shader and check for errors
	glCompileShader(fragment_shader);
	let mut success = 0;
	glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
	if success == 0 {
		let mut v: Vec<u8> = Vec::with_capacity(1024);
		let mut log_len = 0_i32;
		glGetShaderInfoLog(
			fragment_shader,
			1024,
			&mut log_len,
			v.as_mut_ptr().cast(),
		);
		v.set_len(log_len.try_into().unwrap());
		panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
	}
	
	// create program from shader and fragment shader
	let shader_program = glCreateProgram();
	assert_ne!(shader_program, 0);
	glAttachShader(shader_program, vertex_shader);
	glAttachShader(shader_program, fragment_shader);
	glLinkProgram(shader_program);
	let mut success = 0;
	// check that program links ok
	glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
	if success == 0 {
		let mut v: Vec<u8> = Vec::with_capacity(1024);
		let mut log_len = 0_i32;
		glGetProgramInfoLog(
			shader_program,
			1024,
			&mut log_len,
			v.as_mut_ptr().cast(),
		);
		
		v.set_len(log_len.try_into().unwrap());
		panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
	}
	// clear the shaders from memory
	glDeleteShader(vertex_shader);
	glDeleteShader(fragment_shader);
	
	glUseProgram(shader_program);
	
  }
  
  let mut glmain = glbaby::GLMain::new();
  
  glmain.add_node(Box::new(glbaby::GlobalGL::new()));
  
  glmain.print_structure();
  
  let mut r = 0;
  
  'main_loop: loop {
	  while let Some(event) = sdl.poll_events().and_then(Result::ok){
		match event {
			Event::Quit(_) => break 'main_loop,
			_ => (),
		}
	  }
	
	unsafe {
		glClear(GL_COLOR_BUFFER_BIT);
		glDrawArrays(GL_TRIANGLES, 0, 3);
	}
	
	//println!("ABC");
	glmain.modify(0, glbaby::SocketOperation::SETLATCHED, r);
	glmain.draw();
	
	r += 1;
	if r > 1000{
		r = 0;
	}
	
	win.swap_window();
  }
}