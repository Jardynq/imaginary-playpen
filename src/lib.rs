use std::sync::Arc;
use std::sync::Mutex;
use std::cell::RefCell;
use std::rc::Rc;

#[macro_use]
mod utils;
mod input;
mod shader;
mod canvas;
mod math;
mod javascript;
use javascript::Javascript;

use macro_derive::*;

use log::Level;
use log::info;
use log::debug;
use log::error;
use log::trace;
use log::warn;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    Window, Document, Element, HtmlElement, Node, 
    Navigator,
    console,
    WebGl2RenderingContext as WebGl2,
    
    HtmlDivElement, HtmlParagraphElement, HtmlImageElement,
    HtmlCanvasElement, HtmlVideoElement,
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;











#[wasm_bindgen]
pub struct Application {
    js: Javascript,
    input: Arc<Mutex<input::Handler>>,

    custom_framerate: (bool, f64),
}
#[wasm_bindgen]
impl Application {
    pub fn new() -> Self {
        let mut js = Javascript::default();
        Application {
            js: js.clone(),
            // Is temp
            //input: Arc::new(Mutex::new(input::Handler::new(&document.query_selector("#z-canvas").unwrap().unwrap()))),
            input: Arc::new(Mutex::new(input::Handler::default())),
            custom_framerate: (false, 60.0),
        }
    }

    pub fn init(&mut self) {
        // TODO: Wrap the closure in an error handler that can pretty print errors
        // instead of printing inside the libraries.
        let mut closure = || -> Result<(), JsValue> {
            self.input.lock().unwrap().hook(&self.js.window().unwrap());
            
            let (z_canvas, z_context) = self.js.get_canvas_2d("#z-canvas").unwrap();
            z_canvas.set_width(720);
            z_canvas.set_height(720);

            let (w_canvas, w_context) = self.js.get_canvas_webgl2("#w-canvas").unwrap();
            w_canvas.set_width(720);
            w_canvas.set_height(720);
            w_canvas.style().set_property("width", "400");
            w_canvas.style().set_property("height", "400");
            info!("{}, {}", w_context.drawing_buffer_width(), w_context.drawing_buffer_height());
            w_context.viewport(0, 0, w_context.drawing_buffer_width(), w_context.drawing_buffer_height());
            //w_context.enable(WebGl2::SCISSOR_TEST);
            //w_context.scissor(0, 10, 60, 60);


            //canvas.set_width(1000);
            //canvas.set_height(1000);

            // TODO: catch errors: if browser does not suport webg2
            // TODO: error handling is dogshit.
            // I get an eror an console just tells me undefined. FIX.

            let mut w_shader = shader::Shader::create(&w_context).unwrap();
            w_shader.attach(shader::ShaderType::Vertex, include_str!("./shader/vert.glsl"));
            // Allwo fro runtime updating of sahders
            w_shader.attach(shader::ShaderType::Fragment, include_str!("./shader/fractal.frag"));
            w_shader.link();

            
            w_shader.bind();

            let vertices: Vec<f32> = vec![
                -1.0, -1.0, 0.0, 
                 1.0, -1.0, 0.0, 
                 -1.0,  1.0, 0.0,
                 1.0,  1.0, 0.0, 
            ];
            let indices: Vec<u16> = vec![
                0, 1, 2,
                2, 1, 3,
            ];
            let texcoords: Vec<f32> = vec![
                 0.0,  0.0,
                 1.0,  0.0, 
                 0.0,  1.0,
                 1.0,  1.0,
            ];
            let mut vao = shader::VertexArray::create(&w_context).unwrap();
            vao.bind();
            vao.add_buffer(0, 3, &vertices);
            vao.add_buffer(1, 2, &texcoords);
            vao.add_index_buffer(&indices);

            
            let print_webgl_error = |context: &WebGl2| {
                match context.get_error() {
                    WebGl2::NO_ERROR => info!("No error."),
                    WebGl2::INVALID_ENUM => info!("Invalid enum.."),
                    WebGl2::INVALID_VALUE => info!("Invalid value."),
                    WebGl2::INVALID_OPERATION => info!("Invalid Operation."),
                    WebGl2::INVALID_FRAMEBUFFER_OPERATION => info!("Invalid framebuffer operation."),
                    WebGl2::OUT_OF_MEMORY => info!("Out of memory."),
                    WebGl2::CONTEXT_LOST_WEBGL => info!("Context lost webgl."),
                    _ => info!("Unknown error."),
                }
            };

            // This should not be needed.
            // TODO: figure out when the error is reported. (use log and try to get errror after every gl call).
            w_shader.bind();
            let mut ubo = shader::UniformBufferObject::create(&w_context).unwrap();
            ubo.bind();
            w_shader.bind_uniform_block("NodesBlock", &mut ubo);
            info!("'{}' size: {}", ubo.name, ubo.size);
            unsafe {
                global_ubo = Some(std::sync::Arc::new(ubo));
            }
            eval_rs(JsValue::from_str("z"));
            let ubo = unsafe { global_ubo.as_ref().unwrap().clone() };
            

            let z_texture = w_context.create_texture().unwrap();
            w_context.active_texture(WebGl2::TEXTURE0);
            w_context.bind_texture(WebGl2::TEXTURE_2D, Some(&z_texture));
            w_context.tex_parameteri(WebGl2::TEXTURE_2D, WebGl2::TEXTURE_WRAP_S, WebGl2::CLAMP_TO_EDGE as i32);
            w_context.tex_parameteri(WebGl2::TEXTURE_2D, WebGl2::TEXTURE_WRAP_T, WebGl2::CLAMP_TO_EDGE as i32);
            w_context.tex_parameteri(WebGl2::TEXTURE_2D, WebGl2::TEXTURE_MIN_FILTER, WebGl2::LINEAR as i32);
            w_shader.uniform1i("tex_sampler", 0);

            let band_texture = w_context.create_texture().unwrap();
            w_context.active_texture(WebGl2::TEXTURE1);
            w_context.bind_texture(WebGl2::TEXTURE_2D, Some(&band_texture));
            w_context.tex_parameteri(WebGl2::TEXTURE_2D, WebGl2::TEXTURE_WRAP_S, WebGl2::CLAMP_TO_EDGE as i32);
            w_context.tex_parameteri(WebGl2::TEXTURE_2D, WebGl2::TEXTURE_WRAP_T, WebGl2::CLAMP_TO_EDGE as i32);
            w_context.tex_parameteri(WebGl2::TEXTURE_2D, WebGl2::TEXTURE_MIN_FILTER, WebGl2::LINEAR as i32);
            w_shader.uniform1i("texture_sampler", 1);


            let band_image = self.js.get_element::<HtmlImageElement>("#band-image").unwrap();
            w_context.tex_image_2d_with_u32_and_u32_and_html_image_element(
                WebGl2::TEXTURE_2D, 
                0,
                WebGl2::RGBA as i32,
                WebGl2::RGBA,
                WebGl2::UNSIGNED_BYTE,
                &band_image,
            ).unwrap();







            let video_elem = self.js.get_element::<HtmlVideoElement>("#video").unwrap();
            let (mut video_width, mut video_height) = (0.0, 0.0);


            let main_stream: Arc<Mutex<Option<web_sys::MediaStream>>> = Arc::new(Mutex::new(None));
            let stream = main_stream.clone();

             match &self.js.navigator().unwrap().media_devices() {
                Ok(devices) => {
                    let mut constraint = web_sys::MediaStreamConstraints::new();
                    constraint.audio(&JsValue::from(false));
                    constraint.video(&JsValue::from(true));
                    constraint.picture(true);
                    match devices.get_user_media_with_constraints(&constraint) {
                        Ok(media) => {
                            let temp_callback = Closure::wrap(Box::new(move |media_stream: JsValue| {
                                let mut lock = main_stream.lock().unwrap();
                
                                match media_stream.clone().dyn_into::<web_sys::MediaStream>() {
                                    Ok(media_stream) => *lock = Some(media_stream),
                                    Err(error) => match error.as_string() {
                                        Some(error) => error!("Media stream not available. You need a secure connection. Error info:\n{}", error),
                                        None => error!("Media stream not available. You need a secure connection."),
                                    }
                                }
                
                                let window = web_sys::window().expect("no global window exists.");
                                let document = window.document().expect("window should have a document.");
                
                                let video_elem = document.query_selector("#video").unwrap().unwrap()
                                    .dyn_into::<web_sys::HtmlVideoElement>().unwrap();
                                video_elem.set_src_object(Some(&(lock.as_ref().unwrap())));
                            }) as Box<dyn FnMut(JsValue)>);
                            let temp_error = Closure::wrap(Box::new(move |error: JsValue| {
                                warn!("Media denied");
                            }) as Box<dyn FnMut(JsValue)>);
                            
                            media.then(&temp_callback).catch(&temp_error);
                            temp_callback.forget();
                            temp_error.forget()
                        },
                        Err(error) => match error.as_string() {
                            Some(error) => error!("Media stream not available. You need a secure connection. Error info:\n{}", error),
                            None => error!("Media stream not available. You need a secure connection."),
                        }
                    };
                }
                Err(error) => match error.as_string() {
                    Some(error) => error!("Media stream not available. You need a secure connection. Error info:\n{}", error),
                    None => error!("Media stream not available. You need a secure connection."),
                }
            };







            //self.input.update();
            let other = self.input.clone();

            let self_loop = Rc::new(RefCell::new(None));
            let main_loop = self_loop.clone();

            let mut current_frametime = self.js.time_now();
            let mut last_frametime = current_frametime;
            let mut delta_time = 1.0;
            let (mut xpos, mut ypos) = (0.0, 0.0);
            let (mut xpos2, mut ypos2) = (0.0, 0.0);
            let (mut xvel, mut yvel) = (0.0, 0.0);
            let (mut xzoom, mut yzoom) = (1.0, 1.0);
            let (mut freeze_x, mut freeze_y) = (0.0, 0.0);
            let mut freeze_state = 0; // 0 none, 1 freeze x, 2 freeze y 
            let mut fractal_count = 1.0;
            let mut escape_radius = 2.0;
            let mut max_iters = 200;

            let mut js = self.js.clone();

            let (mut x_old, mut y_old) = (0.0, 0.0);


            let req_anim_frame = |callback: &Closure<dyn FnMut(f64)>| {
                web_sys::window().expect("no global `window` exists")
                    .request_animation_frame(callback.as_ref().unchecked_ref())
                    .expect("should register `requestAnimationFrame` OK");
            };
            *main_loop.borrow_mut() = Some(Closure::wrap(Box::new(move |elapsed: f64| {
                let req_token = req_anim_frame(self_loop.borrow().as_ref().unwrap());

                delta_time = (current_frametime - last_frametime) / 1000.0;
                last_frametime = current_frametime;
                current_frametime = js.time_now();

                let mut lock = other.lock().unwrap();
                lock.update();


                let time = (js.time_now() / 1000.0);
                //console::clear();
                //console::log_6(
                //    &JsValue::from(lock.vkey_active("KeyG")),
                //    &JsValue::from(lock.vbutton_down(input::MouseButton::Left)),
                //    &JsValue::from(lock.vbutton_held(input::MouseButton::Left)),
                //    &JsValue::from(lock.vbutton_pressed(input::MouseButton::Left)),
                //    &JsValue::from(lock.vbutton_released(input::MouseButton::Left)),
                //    &JsValue::from(lock.vbutton_inactive(input::MouseButton::Left)),
                //);
                //console::log_2(
                //    &JsValue::from(lock.pointer().0),
                //    &JsValue::from(lock.pointer().1),
                //);
                //console::log_2(
                //    &JsValue::from(lock.pointer_delta().0),
                //    &JsValue::from(lock.pointer_delta().1),
                //);

                let (x, y) = lock.pointer_relative(&z_canvas);
                if lock.vbutton_down(input::MouseButton::Left) {
                    z_context.begin_path();
                    z_context.set_image_smoothing_enabled(false);
                    z_context.set_line_cap("round");
                    z_context.set_stroke_style(&JsValue::from_str(&format!("#{:02x}{:02x}{:02x}ff", ((time * 0.5).sin() * 255.0 + 0.75) as u8, ((time * 0.5).cos() * 255.0 + 0.75) as u8, ((time * 0.5 + 0.75).tan() * 255.0) as u8)));
                    z_context.set_line_width(10.0);

                    z_context.move_to(x_old, y_old);
                    z_context.line_to(x, y);
                    /*if lock.vkey_down("ShiftLeft") {
                        if dx.abs() > dy.abs() && freeze_state != 2 {
                            freeze_y = y;
                            freeze_state = 2;
                        }
                        if dy.abs() > dx.abs() && freeze_state != 1 {
                            freeze_x = x;
                            freeze_state = 1;
                        }

                        if freeze_state == 1 {
                            z_context.move_to(freeze_x, y - dy);
                            z_context.line_to(freeze_x, y);
                        } else if freeze_state == 2 {
                            z_context.move_to(x - dx, freeze_y);
                            z_context.line_to(x, freeze_y);
                        } else {
                            freeze_y = y;
                            freeze_x = x;
                        }
                    } else {
                        freeze_y = y;
                        freeze_x = x;
                        freeze_state = 0;
                        z_context.move_to(x - dx, y - dy);
                        z_context.line_to(x, y);
                    }*/

                    z_context.stroke();
                }
                let xy_old = lock.pointer_relative(&z_canvas);
                x_old = xy_old.0;
                y_old = xy_old.1;
                
                
                



                
                
                //let z_image = z_context.get_image_data(0.0, 0.0, z_canvas.width().into(), z_canvas.height().into()).unwrap();

                w_shader.bind();
                vao.bind();
                
                w_context.bind_texture(WebGl2::TEXTURE_2D, Some(&z_texture));
                match &*stream.lock().unwrap() {
                    Some(_) => {
                        w_context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_video_element(
                            WebGl2::TEXTURE_2D, 
                            0,
                            WebGl2::RGBA as i32,
                            video_elem.width() as i32,
                            video_elem.height() as i32,
                            0,
                            WebGl2::RGBA,
                            WebGl2::UNSIGNED_BYTE,
                            &video_elem,
                        ).unwrap();
                    },
                    None => {
                        w_context.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
                            WebGl2::TEXTURE_2D, 
                            0,
                            WebGl2::RGBA as i32,
                            WebGl2::RGBA,
                            WebGl2::UNSIGNED_BYTE,
                            &z_canvas,
                        ).unwrap();
                    }
                }

                
                w_context.active_texture(WebGl2::TEXTURE0);
                w_context.bind_texture(WebGl2::TEXTURE_2D, Some(&z_texture));
                w_context.active_texture(WebGl2::TEXTURE1);
                w_context.bind_texture(WebGl2::TEXTURE_2D, Some(&band_texture));
                
                ubo.bind();

                //if (isPowerOf2(image.width) && isPowerOf2(image.height)) {
                //    // Yes, it's a power of 2. Generate mips.
                //    gl.generateMipmap(gl.TEXTURE_2D);
                // } else {
                    // No, it's not a power of 2. Turn off mips and set
                    // wrapping to clamp to edge

                


            

                if lock.vkey_down("ArrowLeft") {
                    xvel = -0.5;
                }
                else if lock.vkey_down("ArrowRight") {
                    xvel = 0.5;
                }
                if lock.vkey_down("ArrowUp") {
                    yvel = -0.5;
                }
                else if lock.vkey_down("ArrowDown") {
                    yvel = 0.5;
                }
                if lock.vkey_down("ShiftLeft") {
                    xvel = xvel * 2.0;
                    yvel = yvel * 2.0;
                }

                if lock.vkey_down("Slash") {
                    xzoom += 1.0 * xzoom * delta_time;
                    yzoom += 1.0 * yzoom * delta_time;
                } else if lock.vkey_down("Period") {
                    xzoom -= 1.0 * xzoom * delta_time;
                    yzoom -= 1.0 * yzoom * delta_time;
                    if yzoom <= 0.0 {
                        yzoom = 0.0001;
                    }
                    if xzoom <= 0.0 {
                        xzoom = 0.0001;
                    }
                }

                if lock.vkey_down("KeyA") {
                    xpos2 += -0.1 * delta_time / xzoom;
                }
                else if lock.vkey_down("KeyD") {
                    xpos2 += 0.1 * delta_time / xzoom;
                }
                if lock.vkey_down("KeyW") {
                    ypos2 += -0.1 * delta_time / yzoom;
                }
                else if lock.vkey_down("KeyS") {
                    ypos2 += 0.1 * delta_time / yzoom;
                }

                xpos += xvel * delta_time / yzoom;
                ypos += yvel * delta_time / xzoom;
                xvel = 0.0;
                yvel = 0.0;
                w_shader.uniform2f("view_offset", xpos as f32, ypos as f32);
                w_shader.uniform2f("offset", xpos2 as f32, ypos2 as f32);
                w_shader.uniform2f("z_size", z_canvas.width() as f32, z_canvas.height() as f32);
                w_shader.uniform2f("w_size", w_canvas.width() as f32, w_canvas.height() as f32);
                w_shader.uniform2f("zoom", xzoom as f32, yzoom as f32);

                unsafe {
                    w_shader.uniform1i("nodes_count", global_nodes_count.as_ref().unwrap().get() as i32);
                }

                if lock.vkey_down("KeyO") {
                    fractal_count += 0.1 * delta_time;
                } else if lock.vkey_down("KeyP") {
                    fractal_count -= 0.1 * delta_time;
                }
                if lock.vkey_down("KeyU") {
                    escape_radius += 0.1 * delta_time;
                } else if lock.vkey_down("KeyI") {
                    escape_radius -= 0.1 * delta_time;
                }
                if lock.vkey_down("KeyT") {
                    max_iters += 1;
                } else if lock.vkey_down("KeyY") {
                    max_iters -= 1;
                }



                w_shader.uniform1f("time", (js.time_now() / 1000.0) as f32);
                w_shader.uniform1i("max_iterations", max_iters);
                w_shader.uniform2f("offset", xpos as f32, ypos as f32);
                w_shader.uniform2f("center", xpos2 as f32, ypos2 as f32);
                w_shader.uniform2f("zoom", xzoom as f32, yzoom as f32);
                w_shader.uniform1f("aspect_ratio", 1.0); // TODO:
                w_shader.uniform1f("escape_radius", escape_radius as f32);
                w_shader.uniform1f("fractal_count", (fractal_count * 2.0) as f32);
                w_shader.uniform1i("flags", 0x0 | 0x4);
                w_shader.uniform1i("coloring", 0x1);


                w_context.clear_color(1.0, 0.0, 0.0, 1.0);
                w_context.clear(WebGl2::COLOR_BUFFER_BIT);

                w_context.draw_elements_with_i32(
                    WebGl2::TRIANGLES,
                    indices.len() as i32,
                    WebGl2::UNSIGNED_SHORT,
                    0,
                );
                //w_context.draw_arrays(
                //    WebGl2::TRIANGLES,
                //    0,
                //    (vertices.len() / 3) as i32,
                //);
            }) as Box<dyn FnMut(f64)>));


            //Interval::new(100, self_loop.borrow().as_ref().unwrap()).forget();

            //let p: HtmlParagraphElement = self.document.create_element("p")?.into();
            //let div: HtmlDivElement = self.document.create_element("div")?.into();
            /*
            let div = self.document.create_element("div")?;
            let p = self.document.create_element("p")?;

            p.set_inner_html("ahahah fck");
            p.set_attribute("id", "tester")?;
            console::log_1(&JsValue::from_str(&format!("{}", p.inner_html())));
            console::log_1(&JsValue::from_str(&format!("{}", p.inner_html())));
            console::log_1(&JsValue::from_str(&format!("{}", p.get_attribute("id").unwrap())));
        
            div.append_child(&p)?;
            self.body.append_child(&div)?;
        
            let new_p = self.document.query_selector("#tester")?.unwrap();
            console::log_1(&JsValue::from_str(&format!("{}", new_p.inner_html())));
            */
            req_anim_frame(main_loop.borrow().as_ref().unwrap());
            Ok(())
        };
        match closure() {
            Ok(_) => (),
            Err(error) => error!("initialization error:\n{:?}", error),
        };
    }
    pub fn start() {

    }

    pub fn update(&mut self) {

    }


    pub fn terminate(mut self) {
        //self.input.unhook();
    }
}










//#[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn create() -> Result<Application, JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    match console_log::init_with_level(Level::Debug) {
        Ok(_) => (),
        Err(error) => unsafe { console::error_1(&JsValue::from_str(&error.to_string())) },
    }
    //let f = Function::new_with_args("event", "
    //    console.log(event);
    //    console.log(publicVar);
    //");
    //f.call1(&JsValue::null(), &JsValue::from("hi im a rustianssesses"));
    //window.add_event_listener_with_callback("click", &f);
    //EventListener::add(EventType::click);
    //window.set_onload(Some(Application::new(window).init));
    let app = Application::new();
    //loop {
    //    console::log_1(&"oh noes".into());
    //}
    //let z_canvas = document.query_selector("#z-canvas")?.unwrap();
//
    //EventListener::new(&z_canvas, "mousemove", move |event| {
    //    let mouse_event = MouseEvent::from(JsValue::from(event));
    //    let position = (
    //        mouse_event.screen_x() as f64,
    //        mouse_event.screen_y() as f64,
    //    );
    //    console::log_1(&JsValue::from_f64(position.0));
    //}).forget();

    
    // Pass the app so it does not get dropped.
    Ok(app)
}




// TODO, this shouldnt be an enum, but rather a functions that maps.
#[derive(Clone, Debug)]
enum Variable {
}
impl math::KeywordType for Variable {
    fn serialize(name: &str) -> u32 {
        match name {
            "time" => 0,
            "z" => 1,
            "zr" => 2,
            "zi" => 3,
            "uv" => 4,
            "uvx" => 5,
            "uvy" => 6,
            "zoom" => 7,
            "zoomx" => 8,
            "zoomy" => 9,
            "offset" => 10,
            "offsetx" => 11,
            "offsety" => 12,
            "e" => 13,
            "pi" => 14,
            "phi" => 15,
            _ => 0,
        }
    }
}
impl Variable {
    fn keywords() -> Vec<math::Keyword> {
        math::Keyword::vars(&vec![
            "time",
            "z",
            "zr",
            "zi",
            "uv",
            "uvx",
            "uvy",
            "zoom",
            "zoomx",
            "zoomy",
            "offset",
            "offsetx",
            "offsety",
            "e",
            "pi",
            "phi",
        ])
    }
}


// TODO, this shouldnt be an enum, but rather a functions that maps.
#[derive(Clone, Debug)]
enum Function {
}
impl math::KeywordType for Function {
    fn serialize(name: &str) -> u32 {
        match name {
            "sin" => 0,
            "cos" => 1,
            "tan" => 2,
            "sqrt" => 3,
            "log" => 4,
            "exp" => 5,
            "abs" => 6,
            "arg" => 7,
            "round" => 8,
            "ceil" => 9,
            "floor" => 10,
            "rand" => 11,
            _ => 0,
        }
    }
}
impl Function {
    fn keywords() -> Vec<math::Keyword> {
        math::Keyword::funcs(&vec![
            "sin",
            "cos",
            "tan",
            "sqrt",
            "log",
            "exp",
            "abs",
            "arg",
            "round",
            "ceil",
            "floor",
            "rand",
        ])
    }
}


static mut global_ubo: Option<std::sync::Arc<shader::UniformBufferObject>> = None;
static mut global_nodes_count: Option<std::sync::Arc<std::cell::Cell<usize>>> = None;


#[wasm_bindgen]
pub fn eval_rs(input: JsValue) -> JsValue {
    let mut keywords = Variable::keywords();
    keywords.append(&mut Function::keywords());
    let mut expr = math::Expression::parse(&input.as_string().unwrap(), &keywords);
    expr.optimize();
    let tree = match &expr.tree {
        Some(tree) => tree,
        None => {
            match &expr.error {
                Some(error) => error!("{}", expr.pretty_error()),
                None => (),
            }
            return JsValue::from_str("");
        }
    };
    let mut nodes = expr.serialize::<Variable, Function>();
    info!("{:#?}", nodes);
    
    unsafe {
        global_ubo.as_ref().unwrap().set_data(&nodes);
        match global_nodes_count {
            Some(_) => (),
            None => {
                global_nodes_count = Some(Arc::new(std::cell::Cell::new(0)));
            }
        }
        global_nodes_count.as_ref().unwrap().set(nodes.len());
    }

    JsValue::from_str(&tree.to_string())
}
/*
#[wasm_bindgen]
pub fn print_rs(input: JsValue) -> JsValue {
    let tree = math::parse::<Variable, Function>(&input.as_string().unwrap());
    JsValue::from_str(&tree.to_string())
}
#[wasm_bindgen]
pub fn token_rs(input: JsValue) -> JsValue {
    let tokens = math::analyse_lex::<Variable, Function>(&input.as_string().unwrap());
    JsValue::from_str(&format!("{:#?}", tokens))
}*/

// If i want to use callbacks/listeners, then i've got to add more glue!
// the main thread cannot block!!
