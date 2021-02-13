mod utils;

use wasm_bindgen::prelude::*;
use web_sys::FileReader;
use web_sys::Blob;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct File {
    content: wasm_bindgen::JsValue,
}

#[wasm_bindgen]
pub struct Converter {
    blob: Blob,
    file_reader: FileReader,
    file: File,
    ready: bool
}

#[wasm_bindgen]
impl Converter {
    pub fn new_reader(&mut self) {
        self.file_reader = FileReader::new().unwrap();
    }

    pub fn read_to_arraybuffer(&mut self) {
        match self.file_reader.read_as_array_buffer(&self.blob) {
            Err(e) => {
                let error;
                match wasm_bindgen::JsValue::as_string(&e) {
                    Some(s) => error = s,
                    None => error = "".to_string(),
                }
                log(&*error)
            },
            Ok(_) => self.ready = true,
        }

        self.log_out();

        match self.file_reader.result() {
            Err(e) => {
                let error;
                match wasm_bindgen::JsValue::as_string(&e) {
                    Some(s) => error = s,
                    None => error = "".to_string(),
                }
                log(&*error)
            },
            Ok(a) => self.file.content = a,
        }
    }

    pub fn log_out(&self) {
        log(&*format!("{}", self.ready));
    }
}