#![allow(non_snake_case)]
#[macro_use]

extern crate serde_derive;

extern crate sdl2;
extern crate gl;
extern crate xml;
extern crate serde;
extern crate serde_json;
extern crate stb_image;
extern crate rand;
extern crate fp_rust;
extern crate bigint;

pub mod render_gl;

pub mod mesh;

pub mod form;

pub mod gamefield;

use std::ffi::{CString, CStr};


fn main() {

    let background: form::Background = serde_json::from_str(&std::fs::read_to_string("./Data/Screen1.xml").unwrap()).unwrap();
    

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let width = (video_subsystem.current_display_mode(0).unwrap().h*3/4) as u32;
    let height = (video_subsystem.current_display_mode(0).unwrap().h*3/4) as u32;
    let _window = video_subsystem
        .window("Game", width, height)
	.opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = _window.gl_create_context().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();
    let mut state;

    let mut mouse_clicked:  bool = false;
    let mut mouse_pos_x : f32;
    let mut mouse_pos_y : f32;
    let mut GF : gamefield::GameField = gamefield::GameField::create(0,0);

    unsafe {
	gl::Viewport(0, 0, width as i32, height as i32);
        gl::ClearColor(background.r, background.g, background.b, background.a);
    }

    let vert_shader = render_gl::Shader::from_vert_source(&CString::new(include_str!("./shaders/triangle.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(&CString::new(include_str!("./shaders/triangle.frag")).unwrap()).unwrap();
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

   
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
    	gl::GenBuffers(1, &mut vbo);
    }

    let mut hit_target :bool = false;
    
    let mut MeshMap = std::collections::HashMap::new();
    let mut fields  = vec![];

    let mut j = 0.0;
    while j < 8.0 {
	let mut i = 0.0;
	while i < 8.0 {
	    fields.push(form::Form::createQua(-1.0+1.0/8.0+i/4.0,1.0- (1.0/8.0+j/4.0) ,1.0/4.0,1.0/4.0,1.0/4.0));
	    i+=1.0;
	}
	j+=1.0;
    }

    MeshMap.entry("Green".to_string()).or_insert(mesh::Mesh::create("./Data/Img/green.png".to_string()));
    MeshMap.entry("Blue".to_string()).or_insert(mesh::Mesh::create("./Data/Img/blue.png".to_string()));
    MeshMap.entry("Board".to_string()).or_insert(mesh::Mesh::create("./Data/Img/field.png".to_string()));
    let mut change = true;

    'main: loop {
		
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
		sdl2::event::Event::MouseButtonUp { mouse_btn : sdl2::mouse::MouseButton::Left, ..} => { mouse_clicked = true;},
	        _ => {}
            }
	       
        }

	
	
        if mouse_clicked {
            state = event_pump.mouse_state();
	    mouse_pos_x = state.x() as f32 / (width/2) as f32 -1.0;
	    mouse_pos_y = -1.0* (state.y() as f32 / (height/2) as f32 -1.0);
	    let mut i = 0;
	    for field  in &mut fields {
		if field.hit(mouse_pos_x,mouse_pos_y) {
		    hit_target = true;
		    break;
		}
		i+=1;
		
	    }
	    if hit_target && GF.set(i) {change = true;}
	    mouse_clicked = false;
	    hit_target = false;
        }
	
	if change {
	    shader_program.set_used();
	    MeshMap[&"Board".to_string()].draw(&fields[0].get_info());
	    MeshMap[&"Board".to_string()].draw(&fields[0].get_info());
	    MeshMap[&"Board".to_string()].draw(&fields[0].get_info());
	    let mut index = 0;
	    for field in &fields {
	    	MeshMap[&"Board".to_string()].draw(&field.get_info());
		let c = GF.get_content(index);
	    	if c == 1 {MeshMap[&"Green".to_string()].draw(&field.get_info());}
	    	if c == 2 {MeshMap[&"Blue".to_string()].draw(&field.get_info());}
		index+=1;
	    }
            _window.gl_swap_window();
	    unsafe {gl::Clear(gl::COLOR_BUFFER_BIT);}
	}
	change = GF.ai_action();
    }
}

