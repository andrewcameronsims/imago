extern crate image;
extern crate img_hash;

extern crate libc;
use std::ffi::{CStr,CString};
use img_hash::{HasherConfig,HashAlg};

#[no_mangle]
pub extern fn hash_image(filepath: *const libc::c_char) -> *const libc::c_char {
  let filepath_cstr = unsafe { CStr::from_ptr(filepath) };
  let filepath_string = filepath_cstr.to_str().unwrap();
  let image = image::open(&filepath_string).unwrap();

  let hasher = HasherConfig::new().to_hasher();
  let hash = hasher.hash_image(&image).to_base64();

  CString::new(hash).unwrap().into_raw()
}

#[no_mangle]
pub extern fn hamming_distance(left: *const libc::c_char, right: *const libc::c_char) -> u32 {
  let hasher = HasherConfig::new().to_hasher();

  let left_cstr = unsafe { CStr::from_ptr(left) };
  let left_str = left_cstr.to_str().unwrap();
  let left_image = image::open(&left_str).unwrap();
  let left_hash = hasher.hash_image(&left_image);

  let right_cstr = unsafe { CStr::from_ptr(right) };
  let right_str = right_cstr.to_str().unwrap();
  let right_image = image::open(&right_str).unwrap();
  let right_hash = hasher.hash_image(&right_image);

  left_hash.dist(&right_hash)
}

