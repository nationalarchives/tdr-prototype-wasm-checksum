extern crate wasm_bindgen_futures;
extern crate math;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::future_to_promise;
use web_sys::Blob;
use web_sys::Response;
use sha2::{Sha256, Digest};
use math::round;

#[wasm_bindgen]
pub fn generate_checksum(blob: Blob, callback: js_sys::Function) -> js_sys::Promise {
    future_to_promise(checksum(blob, callback))
}

async fn checksum(blob: Blob, callback: js_sys::Function) -> Result<JsValue, JsValue>{
    let mut hasher = Sha256::new();
    let file_chunk_size: f64 = 10000000.0;
    let blob_size: f64 = blob.size();
    let chunk_count = round::ceil(blob_size / file_chunk_size, 0);

    for x in 0..chunk_count as i32 {
        let start = x as f64 * file_chunk_size;
        // log_1(&JsValue::from_str(&format!("{}",chunk_count)));
        let end = start + file_chunk_size;
        let sliced_blob = blob.slice_with_f64_and_f64(start, end).unwrap();
        let response = Response::new_with_opt_blob(Some(&sliced_blob)).unwrap();
        let array_buffer = response.array_buffer().unwrap();
        let future = JsFuture::from(array_buffer);
        let future_result = future.await?;
        let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&future_result);
        let mut body = vec![0; typebuf.length() as usize];        
        typebuf.copy_to(&mut body);
        hasher.input(&body);
        let percentage: f64 = (x as f64 + 1.0) / chunk_count * 100.0;
        let this = JsValue::NULL;
        let _ = callback.call1(&this, &JsValue::from_f64(percentage));
    }
    let result = hasher.result();
    let result_string = format!("{:x}", result);
    Ok(JsValue::from_str(&result_string))    
}
