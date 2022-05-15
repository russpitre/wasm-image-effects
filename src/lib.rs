use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{ encode, decode };
use image::ImageOutputFormat::Png;
use image::load_from_memory;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded into memory".into());

    img = img.grayscale();
    //img = img.blur(12.23);

    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    return data_url;
}