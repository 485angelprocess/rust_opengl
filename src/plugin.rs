/******************
 OPENGL Rust 
 plugin implementation
 
*******************/

// TODO maybe set as options
// but having them as default is not bad necessarily
pub struct PluginValue{
	data_float: f32,
	data_string: string
	// TODO account for other type data
}

pub struct PluginAttach{
	
}

enum PluginValueType{
	FLOAT, // i.e. default, some float value
	BINARY, // can be 1 or 0
	DISCRETE, // must be whole number
	FILE, // string filename, i.e. hint for file explorer
	STRING // generic string type
	
	// TODO specific gl types 
	// like buffer pointers
	// which could be treated as discrete values
	// but really should be labelled better etc
	
	// TODO video stream input?
	// but that is more a matter of draw order
}

enum PluginDrawFlag{
	REDRAW
}

enum DrawOrderMajor{
	TOP,
	MIDDLE,
	BOTTOM
}

pub struct GLPluginParameter{
	id: u32,
	name: string,
	displayname: string,
	paramtype: PluginValueType,
	visible: bool,
	
	// max inputs from other plugins
	// 	0 - only this plugin can set this value
	// 1
	maxinputs: u32,
	minvalue: f32, 
	maxvalue: f32,
	isinput: bool
}

impl GLPluginParameter{
	pub fn new_float_input(id: u32, name: string, minv: f32, maxv: f32){
		GLPluginParameter{
			id: id,
			name: name,
			displayname: name,
			paramtype: PluginValueType::FLOAT,
			maxinputs: 100,
			minvalue: minv,
			maxvalue: maxv,
			isinput: true
		}
	}
}

pub struct PluginContext{
	values: Vec<PluginValue>, // vector of data
	flags: Vec<plugin_flag> // TODO typedef
	// TODO: window/gl context
	// i think for multiple layer u wanna do whatver
}

pub struct WorkspaceContext{
	// Contains information about gui,
	// available inputs
	// required to allow things like routing inputs
	modules: Vec<u32> // TEMP
}

pub trait GLPlugin{
	fn new() -> GLPlugin{
		/* Create new plugin from base */
	}

	/***********************/
	/** These methods are to help
	/** create 'MIXING' inputs and
	/** such ***************/
	/** i.e. this gets wrapped 
	/** so we can easily generate input/output sockets
	/************************/
	fn GetParameters(&self) -> Option<Vec<GLPluginParameter>>{
		None
	}
	
	/* This may be variable */
	// but state can be handled in plugin
	fn GetNumParams(&self) -> u32{
		if let Some(inputs) = self.GetParameters(){
			inputs.len()
		}
		0
	}
	
	fn GetInputIndices(&self) -> Vec<u32>{
		// i think there's a way to return like an iterator
		// but i don't want to look into that rn
		let v = Vec::new();
		if let Some(inputs) = self.GetParameters(){
			for i in 0..GetNumParams(){
				if (inputs[i].isinput){
					v.push(i);
				}
			}
		}
		v
	}

	/* Get a vector of all the input parameters */
	fn GetInputs(&self) -> Vec<GLPluginParameter>{
		let v = Vec::new();
		if let p = Some(self.GetParameters()){
			for i in self.GetInputIndices(){
				v.push(p[i]);
			}
		}
		v
	}
	
	// Draw order hints
	// LAYER order should be handled in wrapper
	fn GetDrawOrderMajor(&self) -> DrawOrderMajor{
		DrawOrderMajor::Top
	}
	
	fn GetDrawOrderMinor(&self) -> u32{
		0
	}
	
	// TODO get vector of output parameters
	
	/* Get the string name of index */
	fn GetParamName(&self, index: u32) -> string{
		if let Some(inputs) = self.GetParameters(){
			inputs[index].displayname
		}
		"NOTSPECIFIED"
	}
	
	/* Might be more 'usage' but roles can be values or files */
	fn GetParamType(&self, index: u32) -> u32{ 
		if let Some(inputs) = self.GetParameters(){
			inputs[index].paramtype
		}
		-1
	}
	
	fn GetParamVisible(&self, index: u32) -> u32{
		if let Some(inputs) = self.GetParameters(){
			inputs[index].visible
		}
		-1
	}
	
	fn GetParamMax(&self, index: u32) -> f32{
		if let Some(inputs) = self.GetParameters(){
			inputs[index].maxvalue
		}
		32768.0
	}
	
	fn GetParamMin(&self, index: u32) -> f32{
		if let Some(inputs) = self.GetParameters(){
			inputs[index].minvalue
		}
		0.0
	}
	
	fn GetParamFloatValue(&self, index: u32) -> Option<f32>{
		// OVERRIDE
		0.0
	}
	
	// TODO get general display values
	
	/* Draw from context */
	// Plugin wrapper should have field for SHOW UI
	fn draw(&self, ui, ctx :PluginContext, workspace: WorkspaceContext) -> PluginContext{
		// Lookup parameters from context
		// Context should be essentially vector 
		// but ideally with string lookup
		self.drawGeneric(ui, ctx)
	}
	
	fn connect(&self) -> Option<Vec<PluginAttach>>{
		// Return vector of requests to connect inputs and outputs
		None
	}
	
	fn drawGeneric(&self, ui: &mut egui::Ui, ctx: PluginContext, workspace: WorkspaceContext) -> PluginContext{
		// so draw each input
		// add each input value to context
		// if changed set redraw flag
		for i in self.GetInputIndices(){
			match input.paramtype{
				PluginValueType::FLOAT => println!("float value"), // draw slider with label
				_ => println!("Generic draw type not implemented") // TODO print out what type is not implemented
			};
		}
	}
	
	fn gl_setup(&self, ctx){
		/* Override */
		// initial setup of opengl
		// run on 'reset'
	}
	
	fn gl_redraw(&self) -> Option<PluginContext>{
		/* Override */
		None
	}
	
	fn gl_draw(&self, ctx: PluginContext){
		for flag in ctx.flags{
			match flag{
				// TODO get flags from top level file
				PluginDrawFlag::REDRAW: gl_redraw()
			}
		}
	}
	
	// Runs on each frame
	fn Step(&mut self) -> Option<PluginContext>{
		/* Override */
		// Returns context with any values that are changing per frame
		None
	}
	
	fn SetValue(&mut self, index: u32, value: PluginValue){
		// collect mixed in etc. values from changed values
		// and update the actual private values of the struct
		// try to only run for changed values
	}
};

pub struct GlobalBackground{
	r: f32,
	g: f32,
	b: f32,
	a: f32
}

impl GLPlugin for GlobalBackground{
	fn GetParameters(&self) -> Option<Vec<GLPluginParameter>>{
		// The trait is written so this gets called multiple times
		// ideally we don't recall this each time, although if they're all constant it's not really a huge deal
		// it's also called on the EGUI thread so not as timing critical
		let rgba = vec![GLPluginParameter::new_float_input(0, "red", 0.0, 32768.0),
							GLPluginParameter::new_float_input(1, "blue", 0.0, 32768.0),
							GLPluginParameter::new_float_input(2, "green", 0.0, 32768.0),
							GLPluginParameter::new_float_input(3, "alpha", 0.0, 32768.0)];
		Some(rgba)
	}
	
	fn GetParamFloatValue(&self,index:u32) -> Option<f32>{
		// Get the float values of each parameter
		// when asked
		match index{
			0 => Some(self.r),
			1 => Some(self.g),
			2 => Some(self.b),
			3 => Some(self.a),
			_ => None
		}
	}
	
	// TODO stylize method names more consistent lol
	fn gl_redraw(&self){
		unsafe{
			// Set background color correctly
			glClearColor( self.r / 32768.0, self.g / 32768.0, self.b / 32768.0, self.a / 32768.0 );
		}
	}
	
	fn SetValue(&mut self, index: u32, value: PluginValue){
		// This is a pretty simple based on how ffgl does it
		match index{
			0 => {self.r = value.data_float;},
			1 => {self.g = value.data_float;},
			2 => {self.b = value.data_float;},
			3 => {self.a = value.data_float;},
			_ => println!("Tried to write invalid index")
		};
	}
}