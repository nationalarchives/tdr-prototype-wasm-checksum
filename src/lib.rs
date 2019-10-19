extern crate wasm_bindgen_futures;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::future_to_promise;
use sha2::{Sha256, Digest};

#[wasm_bindgen]
pub async fn generate_checksum(blob: web_sys::Blob) -> js_sys::Promise {
    future_to_promise(checksum(blob))
}

async fn checksum(blob: web_sys::Blob) -> Result<JsValue, JsValue>{
    let mut hasher = Sha256::new();
    for x in (0..blob.size() as i32).step_by(5000) {
        let sliced_blob = blob.slice_with_i32_and_i32(x, x+5000).unwrap();
        let response = web_sys::Response::new_with_opt_blob(Some(&sliced_blob)).unwrap();
        let array_buffer = response.array_buffer().unwrap();
        let future = JsFuture::from(array_buffer);
        let future_result = future.await?;
        let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&future_result);
        let mut body = vec![0; typebuf.length() as usize];        
        typebuf.copy_to(&mut body);
        hasher.input(&body);       
    }
    let result = hasher.result();
    let result_string = format!("{:x}", result);
    Ok(JsValue::from_str(&result_string))    
}
