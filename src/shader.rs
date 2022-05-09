use std::collections::HashMap;
use log::{ warn, error };

use web_sys::{
    WebGl2RenderingContext as WebGl2, 
    WebGlProgram,
    WebGlUniformLocation,
    WebGlVertexArrayObject,
    WebGlBuffer,
};




pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    context: WebGl2,
    program: WebGlProgram,
    location_cache: HashMap<String, Option<WebGlUniformLocation>>,
    block_binding: usize,
}
impl Shader {
    pub fn create(context: &WebGl2) -> Option<Self> {
        let program = match context.create_program() {
            Some(program) => program,
            None => {
                error!("Shader::create : Failed to create program.");
                return None;
            }
        };
        Some(Self {
            context: context.clone(),
            program: program,
            location_cache: HashMap::new(),
            block_binding: 0,
        })
    }
    // &sxtr maybe.
    //pub fn define_attributes(&mut self, names: &[String]) {
    //    for (i, name) in names.iter().enumerate() {
    //        let location = self.context.get_attrib_location(&self.program, name);
    //        if location == -1 {
    //            error!("Shader::set_attibutes : Attribute '{}' not found", name));
    //        }
    //        self.context.vertex_attrib_pointer_with_i32(location, 3, WebGl2::FLOAT, false, 0, 0);
    //        self.context.enable_vertex_attrib_array(i);
    //        
    //        // Used for setting custom names for attribz. not uzeful for my system.
    //        //self.context.bind_attrib_location(this.program, attrib[1], attrib[0]);
    //    }
    //}

    pub fn attach(&self, shader_type: ShaderType, source: &str) {
        let (shader_type, shader_name) = match shader_type {
            ShaderType::Vertex => (WebGl2::VERTEX_SHADER, "vertex"), 
            ShaderType::Fragment => (WebGl2::FRAGMENT_SHADER, "fragment"), 
        };

        let shader = match self.context.create_shader(shader_type) {
            Some(shader) => shader,
            None =>  {
                error!("Shader::attach : Failed to create {} shader.", shader_name);
                return;
            }
        };
        self.context.shader_source(&shader, source);
        self.context.compile_shader(&shader);

        let status = self.context
            .get_shader_parameter(&shader, WebGl2::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false);
        if !status {
            let error = self.context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error."));
            error!("Shader::attach : Failed to compile {} shader:\n{}", shader_name, error);
        }

        self.context.attach_shader(&self.program, &shader);
    }
    
    pub fn link(&self) {
        self.context.link_program(&self.program);
        let status = self.context
            .get_program_parameter(&self.program, WebGl2::LINK_STATUS)
            .as_bool()
            .unwrap_or(false);
        
        match status {
            true => (),
            false => {
                let error = self.context
                    .get_program_info_log(&self.program)
                    .unwrap_or_else(|| String::from("Unknown error."));
                error!("Shader::link : Failed to link program:\n{}", error);
            }
        };
    }

    pub fn bind(&self) {
        self.context.use_program(Some(&self.program));
    }
    pub fn unbind(&self) {
        self.context.use_program(None);
    }






    pub fn uniform_location(&mut self, name: &str) -> Option<WebGlUniformLocation> {
        match self.location_cache.get(name) {
            Some(location) => {
                location.clone()
            },
            None => {
                match self.context.get_uniform_location(&self.program, name) {
                    Some(found) => {
                        self.location_cache.insert(name.to_owned(), Some(found.clone()));
                        Some(found)
                    },
                    None => {
                        self.location_cache.insert(name.to_owned(), None);
                        warn!("Shader::uniform_location : Uniform '{}' not found", name);
                        None
                    }
                }
            }
        }
    }
    pub fn uniform1f(&mut self, name: &str, value: f32) {
        let location = self.uniform_location(name);
        self.context.uniform1f(location.as_ref(), value);
    }

    pub fn uniform2f(&mut self, name: &str, x: f32, y : f32) {
        let location = self.uniform_location(name);
        self.context.uniform2f(location.as_ref(), x, y);
    }

    pub fn uniform1i(&mut self, name: &str, value: i32) {
        let location = self.uniform_location(name);
        self.context.uniform1i(location.as_ref(), value);
    }

    pub fn bind_uniform_block(&mut self, name: &str,  buffer: &mut UniformBufferObject) {
        let location = self.context.get_uniform_block_index(&self.program, name);
        if location != WebGl2::INVALID_INDEX {
            self.context.uniform_block_binding(&self.program, location, self.block_binding as u32);
            self.context.bind_buffer_base(WebGl2::UNIFORM_BUFFER, self.block_binding as u32, Some(&buffer.internal));
            self.block_binding += 1;

            // TODO: Maybe handle error or something?
            buffer.name = name.to_string();
            buffer.size = match self.context.get_active_uniform_block_parameter(&self.program, location, WebGl2::UNIFORM_BLOCK_DATA_SIZE) {
                Ok(size) => match size.as_f64() {
                    Some(size) => size as usize,
                    None => 0,
                }
                Err(_) => 0,
            };
        }
        else {
            error!("Shader::bind_uniform_block : Uniform block '{}' not found", name);
        }
    }
    /*
    pub fn get_uniform_block_size(&self) -> usize {
        // Cache instead?
        let location = self.context.get_uniform_block_index(&self.program, name);
        match self.context.get_active_uniform_block_parameter(&self.program, location, WebGl2::UNIFORM_BLOCK_DATA_SIZE).dyn_into::<i32>() {
            Some(size) => size as usize,
            None => 0,
        }

    }
    */
}
impl Drop for Shader {
    fn drop(&mut self) {
        self.context.delete_program(Some(&self.program));
    }
}








pub struct VertexArray {
    context: WebGl2,
    internal: WebGlVertexArrayObject,
    buffers: Vec<WebGlBuffer>,
}
impl VertexArray {
    pub fn create(context: &WebGl2) -> Option<Self> {
        let vao = match context.create_vertex_array() {
            Some(vao) => vao,
            None => {
                error!("VertexArray::create : Failed to create vertex array object.");
                return None;
            }
        };
        Some(Self {
            context: context.clone(),
            internal: vao,
            buffers: vec!(),
        })
    }


    pub fn bind(&self) {
        self.context.bind_vertex_array(Some(&self.internal));
    }
    pub fn unbind(&self) {
        self.context.bind_vertex_array(None);
    }

    pub fn add_buffer(&mut self, index: u32, dimensions: i32,data: &Vec<f32>) {
        let buffer = match  self.context.create_buffer() {
            Some(buffer) => buffer,
            None => {
                error!("VertexArray::add_vertex_buffer : Failed to create buffer.");
                return;
            }
        };

        self.context.bind_buffer(WebGl2::ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let array = js_sys::Float32Array::view(&data);
    
            self.context.buffer_data_with_array_buffer_view(
                WebGl2::ARRAY_BUFFER,
                &array,
                WebGl2::STATIC_DRAW,
            );
        };
        self.context.vertex_attrib_pointer_with_i32(index, dimensions, WebGl2::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(index);
        self.buffers.push(buffer);
    }
    pub fn add_index_buffer(&mut self, data: &Vec<u16>) {
        let buffer = match  self.context.create_buffer() {
            Some(buffer) => buffer,
            None => {
                error!("VertexArray::add_index_buffer : Failed to create buffer.");
                return;
            }
        };

        self.context.bind_buffer(WebGl2::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        unsafe {
            let array = js_sys::Uint16Array::view(&data);
    
            self.context.buffer_data_with_array_buffer_view(
                WebGl2::ELEMENT_ARRAY_BUFFER,
                &array,
                WebGl2::STATIC_DRAW,
            );
        };
        self.buffers.push(buffer);
    }
}
impl Drop for VertexArray {
    fn drop(&mut self) {
        for buffer in &self.buffers {
            self.context.delete_buffer(Some(&buffer));
        }
        self.context.delete_vertex_array(Some(&self.internal));
    }
}



pub struct UniformBufferObject {
    pub name: String,
    pub size: usize,

    context: WebGl2,
    internal: WebGlBuffer,
}
impl UniformBufferObject {
    pub fn create(context: &WebGl2) -> Option<Self> {
        let ubo = match context.create_buffer() {
            Some(ubo) => ubo,
            None => {
                error!("UniformBufferObject::create : Failed to create buffer.");
                return None;
            }   
        };
        Some(Self {
            context: context.clone(),
            internal: ubo,
            size: 0,
            name: format!(""),
        })
    }

    pub fn bind(&self) {
        self.context.bind_buffer(WebGl2::UNIFORM_BUFFER, Some(&self.internal));
    }
    pub fn unbind(&self) {
        self.context.bind_buffer(WebGl2::UNIFORM_BUFFER, None);
    }
    // TODO: give it an allocate function that uses buffer_data_i32
    // TODO: use sub_data or map to set data.
    // TODO: use get_buffer_sub_daat to verify. 
    pub fn set_data<T: Clone>(&self, data: &Vec<T>) {
        self.bind();
        unsafe {
            let size = data.len() * std::mem::size_of::<T>();
            let mut raw = std::slice::from_raw_parts(data.as_ptr() as *const u8, size).to_vec();
            if size < self.size {
                raw.append(&mut vec![0_u8; self.size - size]);
            }

            self.context.buffer_data_with_u8_array(
                WebGl2::UNIFORM_BUFFER,
                &raw[..],
                WebGl2::STATIC_DRAW,
            );
        };
    }
}
impl Drop for UniformBufferObject {
    fn drop(&mut self) {
        self.context.delete_buffer(Some(&self.internal));
    }
}

struct MappedBuffer {

}
impl MappedBuffer {

}
