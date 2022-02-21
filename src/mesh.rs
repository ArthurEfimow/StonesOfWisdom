use serde::ser::{Serialize, Serializer,SerializeSeq, SerializeMap, SerializeStruct};
use std::ffi::{CString, CStr};
use std::collections::HashMap;

#[derive(Serialize,Deserialize, Debug,Clone)]
pub struct Texture {
    texture :u32,
    height : i32,
    width : i32
}

impl Texture {
    pub fn create(source : String) -> Texture {
    	let mut width  : i32 = 0;
    	let mut height : i32 = 0;
    	let mut nrChannels : i32 = 0;
    	let mut texture :u32 = 0;
    	unsafe {
	    gl::GenTextures(1, &mut texture as *mut u32);
	    gl::BindTexture(gl::TEXTURE_2D, texture);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

	    let data = stb_image::stb_image::bindgen::stbi_load(CString::new(source).unwrap().as_ptr() as *const i8,&mut width as *mut i32, &mut height as *mut i32, &mut nrChannels as *mut i32, 4);
	
	    if !data.is_null() {
		gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA as u32, gl::UNSIGNED_BYTE, data as *const std::ffi::c_void);
    		gl::GenerateMipmap(gl::TEXTURE_2D);
	    }
	    stb_image::stb_image::bindgen::stbi_image_free(data as *mut std::ffi::c_void);

    	}

	return Texture {texture,height,width};
    }

    pub fn re_init(&mut self,source : String) {
    	self.width   = 0;
    	self.height  = 0;
    	let mut nrChannels  = 0;
    	self.texture = 0;
    	unsafe {
	    gl::GenTextures(1, &mut self.texture as *mut u32);
	    gl::BindTexture(gl::TEXTURE_2D, self.texture);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

	    let data = stb_image::stb_image::bindgen::stbi_load(CString::new(source).unwrap().as_ptr() as *const i8,&mut self.width as *mut i32, &mut self.height as *mut i32, &mut nrChannels as *mut i32, 4);
	
	    if !data.is_null() {
		gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.width, self.height, 0, gl::RGBA as u32, gl::UNSIGNED_BYTE, data as *const std::ffi::c_void);
    		gl::GenerateMipmap(gl::TEXTURE_2D);
	    }
	    stb_image::stb_image::bindgen::stbi_image_free(data as *mut std::ffi::c_void);

    	}
    }

    pub fn bind(&self){
        unsafe {
	    gl::ActiveTexture(gl::TEXTURE0);
	    gl::BindTexture(gl::TEXTURE_2D,self.texture);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteTextures(0, self.texture as * mut u32);	
	}
    }
}

#[derive(Serialize,Deserialize, Debug,Clone)]
pub struct Mesh {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    tex : Texture
}

impl Mesh {
    pub fn create(source : String) -> Mesh {
    	let mut vbo: gl::types::GLuint = 0;
	let mut vao: gl::types::GLuint = 0;
    	unsafe {
    	    gl::GenBuffers(1, &mut vbo);
    	    gl::GenVertexArrays(1, &mut vao);
 	    gl::BindVertexArray(vao);
	    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
	    gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    	    gl::VertexAttribPointer(
            	0, // index of the generic vertex attribute ("layout (location = 0)")
            	2, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	std::ptr::null() // offset of the first component
    	    );
    	    gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
    	    gl::VertexAttribPointer(
            	1, // index of the generic vertex attribute ("layout (location = 1)")
            	4, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	(2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );
    	    gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
    	    gl::VertexAttribPointer(
            	2, // index of the generic vertex attribute ("layout (location = 2)")
            	2, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	(6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );
    	}
	return Mesh {vao,vbo,tex : Texture::create(source)};
    }

    pub fn re_init(&mut self,source : String) {
    	self.vbo = 0;
	self.vao = 0;
    	unsafe {
    	    gl::GenBuffers(1, &mut self.vbo);
    	    gl::GenVertexArrays(1, &mut self.vao);
 	    gl::BindVertexArray(self.vao);
	    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
	    gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    	    gl::VertexAttribPointer(
            	0, // index of the generic vertex attribute ("layout (location = 0)")
            	2, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	std::ptr::null() // offset of the first component
    	    );
    	    gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
    	    gl::VertexAttribPointer(
            	1, // index of the generic vertex attribute ("layout (location = 1)")
            	4, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	(2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );
    	    gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
    	    gl::VertexAttribPointer(
            	2, // index of the generic vertex attribute ("layout (location = 2)")
            	2, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	(6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );
    	}
	self.tex.re_init(source);
    }


    pub fn draw(&self,vertices: &Vec<f32>){
        unsafe {
	    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
    	    gl::BufferData(
            gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
    	    );
      	    gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
	    self.tex.bind();
	    gl::BindVertexArray(self.vao);
	    gl::DrawArrays(gl::TRIANGLES,0,vertices.len() as i32);
	    gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteVertexArrays(0,  self.vao as * mut u32 );
	    gl::DeleteBuffers(0, self.vbo as * mut u32);
	}
    }
}


#[derive(Serialize,Deserialize,Debug)]
pub struct NumDisplay{
    meshes : HashMap<i32, Mesh>,
}

impl NumDisplay {
    pub fn create() -> NumDisplay {
	let mut meshes : HashMap<i32, Mesh> = HashMap::new();
	let mut index : i32 = 0;
	loop {
	    if index > 11 {break;}
	    meshes.insert(index,Mesh::create("./Data/Img/AlphaNum/num".to_string() + &index.to_string() +&".png".to_string()));
	    index += 1;
	}
	return NumDisplay {meshes};
    }
	
    pub fn draw (&mut self,vertices : &Vec<f32>,mut num : i32) {
	if num < 0 {num = 0;}
	if num > 10 {num = 10;}
	self.meshes[&num].draw(&vertices);
    }
}

impl Drop for NumDisplay {
    fn drop(&mut self) {}
}

