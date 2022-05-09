use std::any::type_name;
use log::{ trace, debug, info, warn, error };

use wasm_bindgen::JsCast;
use web_sys::{
    Window, Document, Element, HtmlElement, Node, 
    Navigator, HtmlBodyElement, HtmlCanvasElement,
    Performance, 
    
    CanvasRenderingContext2d,
    ImageBitmapRenderingContext,
    WebGlRenderingContext,
    WebGl2RenderingContext,
};




#[derive(Default, Clone)]
pub struct Javascript {
    window:     Option<Window>,
    document:   Option<Document>,
    body:       Option<HtmlBodyElement>,

    performance:    Option<Performance>,
    navigator:      Option<Navigator>,
}
impl Javascript {
    pub fn reset(&mut self) {
        self.window         = None;
        self.document       = None;
        self.body           = None;

        self.performance    = None;
        self.navigator      = None;
    }

    pub fn get_element<T: JsCast>(&mut self, query: &str) -> Option<T> {
        match self.document() {
            Some(document) => {
                match document.query_selector(query) {
                    Ok(Some(element)) => {
                        match element.dyn_into::<T>() {
                            Ok(element) => Some(element), 
                            Err(error) => {
                                warn!(
                                    "Javascript::get_element : Could not cast '{}' to type '{}'\nWasm error: {:?}", 
                                    query,type_name::<T>(), 
                                    error
                                );
                                None
                            }
                        }
                    }
                    Err(error) => {
                        warn!(
                            "Javascript::get_element : Could not find '{}' on document.\nWasm error: {:?}",
                            query, 
                            error
                        );
                        None
                    }
                    Ok(None) => {
                        warn!(
                            "Javascript::get_element : Could not find '{}' on document.",
                            query
                        );
                        None
                    }
                }
            }
            None => None,
        }
    }

    fn get_canvas_context<T: JsCast>(&mut self, query: &str, context_type: &str) -> Option<(HtmlCanvasElement, T)> {
        match self.get_element::<HtmlCanvasElement>(query) {
            Some(canvas) => {
                match canvas.get_context(context_type) {
                    Ok(Some(context)) => {
                        match context.dyn_into::<T>() {
                            Ok(context) => Some((canvas, context)), 
                            Err(error) => {
                                error!(
                                    "Javascript::get_canvas_context : Could not cast context '{}' to type '{}' for '{}'\nWasm error: {:?}", 
                                    context_type, type_name::<T>(), query, error
                                );
                                None
                            }
                        }
                    }
                    Err(error) => {
                        error!(
                            "Javascript::get_canvas_context : Could not get context '{}' for '{}'\nWasm error: {:?}",
                            context_type, query, error
                        );
                        None
                    }
                    Ok(None) => {
                        error!(
                            "Javascript::get_canvas_context : Could not get context '{}' for '{}'",
                            context_type, query
                        );
                        None
                    }
                }
            },
            None => None,
        }
    }
    pub fn get_canvas_2d(&mut self, query: &str) -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
        self.get_canvas_context::<CanvasRenderingContext2d>(query, "2d")
    }
    pub fn get_canvas_bitmap(&mut self, query: &str) -> Option<(HtmlCanvasElement, ImageBitmapRenderingContext)> {
        self.get_canvas_context::<ImageBitmapRenderingContext>(query, "bitmaprenderer")
    }
    pub fn get_canvas_webgl(&mut self, query: &str) -> Option<(HtmlCanvasElement, WebGlRenderingContext)> {
        self.get_canvas_context::<WebGlRenderingContext>(query, "webgl")
    }
    pub fn get_canvas_webgl2(&mut self, query: &str) -> Option<(HtmlCanvasElement, WebGl2RenderingContext)> {
        self.get_canvas_context::<WebGl2RenderingContext>(query, "webgl2")
    }
    


    pub fn time_now(&mut self) -> f64 {
        match self.performance() {
            Some(performance) => performance.now(),
            None => 0.0,
        }
    }
}


impl Javascript {
    pub fn window(&mut self) -> Option<Window>{
        match &self.window {
            Some(window) => Some(window.clone()),
            None => {
                match web_sys::window() {
                    Some(window) => {
                        self.window = Some(window);
                        self.window.clone()
                    }
                    None => {
                        error!("Javascript::window : No global window found.");
                        None
                    }
                }
            }
        }
    }
    pub fn document(&mut self) -> Option<Document> {
        match &self.document {
            Some(document) => Some(document.clone()),
            None => {
                match self.window() {
                    Some(window) => {
                        match window.document() {
                            Some(document) => {
                                self.document = Some(document);
                                self.document.clone()
                            }
                            None => {
                                error!("Javascript::document : No global document found.");
                                None
                            }
                        }
                    },
                    None => None,
                }
            },
        }
    }
    pub fn body(&mut self) -> Option<HtmlBodyElement> {
        match &self.body {
            Some(body) => Some(body.clone()),
            None => {
                match self.document() {
                    Some(document) => match document.body() {
                        Some(body) => {
                            match body.dyn_into::<HtmlBodyElement>() {
                                Ok(body) => {
                                    self.body = Some(body);
                                    self.body.clone()
                                }
                                Err(error) => {
                                    error!(
                                        "Javascript::body : Could not convert body to HtmlBodyElement.\nnWasm error: {:?}",
                                        error
                                    );
                                    None
                                }
                            }
                        }
                        None => {
                            error!("Javascript::body : No global body found.");
                            None
                        }
                    },
                    None => None,
                }
            },
        }
    }

    pub fn performance(&mut self) -> Option<Performance> {
        match &self.performance {
            Some(performance) => Some(performance.clone()),
            None => {
                match self.window() {
                    Some(window) => match window.performance() {
                        Some(performance) => {
                            self.performance = Some(performance);
                            self.performance.clone()
                        }
                        None => {
                            error!("Javascript::performance : No global performance found.");
                            None
                        }
                    },
                    None => None,
                }
            },
        }
    }
    pub fn navigator(&mut self) -> Option<Navigator> {
        match &self.navigator {
            Some(navigator) => Some(navigator.clone()),
            None => {
                match self.window() {
                    Some(window) => {
                        self.navigator = Some(window.navigator());
                        self.navigator.clone()
                    }
                    None => None,
                }
            },
        }
    }
}
/*
struct Navigator {

}
impl Navigator {
    set_user_media_stream() {

    }
}*/