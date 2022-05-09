#[macro_export]
macro_rules! append_attrs {
    ($document:ident, $el:ident, $( $attr:expr ),* ) => {
        $(
            let attr = $document.create_attribute($attr.0)?;
            attr.set_value(&$attr.1);
            $el.set_attribute_node(&attr)?;
        )*
    }
}
#[macro_export]
macro_rules! remove_attrs {
    ($el:ident, $( $attr:expr ),* ) => {
        $(
            $el.remove_attribute($attr.1);
        )*
    }
}

#[macro_export]
macro_rules! append_text {
    ($document:ident, $el:ident, $text:expr ) => {
        let text = $document.create_text_node($text);
        $el.append_child(&text)?;
    };
}

#[macro_export]
macro_rules! create_element_attrs {
    ($document:ident, $type:expr, $( $attr:expr ),* ) => {{
        let el = $document.create_element($type)?;
        append_attrs!($document, el, $( $attr ),*);
        el}
    }
}

#[macro_export]
macro_rules! append_element_attrs {
    ($document:ident, $parent:ident, $type:expr, $( $attr:expr ),* ) => {
        let el = create_element_attrs!($document, $type, $( $attr ),* );
        $parent.append_child(&el)?;
    }
}

#[macro_export]
macro_rules! append_text_attrs {
    ($document:ident, $parent:ident, $type:expr, $text:expr, $( $attr:expr ),*) => {
        let el = create_element_attrs!($document, $type, $( $attr ),* );
        append_text!($document, el, $text);
        $parent.append_child(&el)?;
    }
}




#[macro_export]
macro_rules! format_js {
    ($($args:tt)*) => {{
        wasm_bindgen::JsValue::from_str(&format!($($args)*))
    }}
}
#[macro_export]
macro_rules! log_error {
    ($($args:tt)*) => {{
        web_sys::console::error_1(&format_js!($($args)*));
    }}
}
#[macro_export]
macro_rules! log_warning {
    ($($args:tt)*) => {{
        web_sys::console::warn_1(&format_js!($($args)*));
    }}
}
macro_rules! log_info {
    ($($args:tt)*) => {{
        web_sys::console::log_1(&format_js!($($args)*));
    }}
}
