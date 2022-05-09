
use crate::shader::Shader;
use web_sys::{
    HtmlCanvasElement,
    WebGl2RenderingContext as WebGl2
};

struct Canvas {
    element: Option<HtmlCanvasElement>,
    context: WebGl2,
    shader: Shader,
}
impl Canvas {
    pub fn set_size(&self, size: (Option<i32>, Option<i32>)) {

    }


    //pub fn render_pre(&mut self, func: Box<dyn FnMut(&mut Self)>) {
    //    func.as_mut()(&self);
    //}
    pub fn render(renderer: Option<Box<dyn FnMut(&Self)>>) {
        
    }
    pub fn render_post() {

    }
}