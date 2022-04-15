mod grayscale;
mod utils;

use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

use grayscale::Grayscale;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Grayscale = 0,
    Quantise = 1,
    WhiteNoise = 2,
    Bayer0 = 3,
    Bayer1 = 4,
    Bayer2 = 5,
    Bayer3 = 6,
    FloydSteinberg = 7,
}

#[wasm_bindgen]
pub fn dither(img: Vec<u8>, width: usize, by: Method) -> Result<Uint8ClampedArray, JsValue> {
    let grayscale = Grayscale::from((img, width));
    match by {
        Method::Grayscale => Ok(grayscale.into()),
        Method::Quantise => Ok(grayscale.quantise().into()),
        Method::WhiteNoise => grayscale.white_noise()
            .map(|g| g.into())
            .map_err(|e| JsValue::from(format!("{e}"))),
        Method::Bayer0 => Ok(grayscale.bayer(0).into()),
        Method::Bayer1 => Ok(grayscale.bayer(1).into()),
        Method::Bayer2 => Ok(grayscale.bayer(2).into()),
        Method::Bayer3 => Ok(grayscale.bayer(3).into()),
        Method::FloydSteinberg => Ok(grayscale.floyd_steinberg().into())
    }
}
