use std::{
  alloc::{alloc, dealloc, Layout},
  ptr::copy_nonoverlapping,
};

pub struct WasmBuffer {
  pub ptr: *mut u8,
  pub len: usize,
  pub owned: bool,
}

impl WasmBuffer {
  pub fn from(ptr: *const u8, len: usize) -> Self {
    return Self {
      ptr: ptr as *mut u8,
      len,
      owned: false,
    };
  }

  pub fn from_owned(ptr: *mut u8, len: usize) -> Self {
    return Self {
      ptr,
      len,
      owned: true,
    };
  }

  pub unsafe fn new(len: usize) -> Self {
    let layout = Layout::array::<u8>(len).unwrap();
    let ptr = alloc(layout);
    return Self::from_owned(ptr, len);
  }

  pub fn from_str(string: &str) -> Self {
    return Self::from(string.as_ptr() as *mut u8, string.len());
  }

  pub unsafe fn new_from_str(string: &str) -> Self {
    let result = Self::new(string.len());
    copy_nonoverlapping(string.as_ptr(), result.ptr, result.len);
    return result;
  }

  pub unsafe fn to_slice<'a>(&self) -> &'a [u8] {
    return std::slice::from_raw_parts(self.ptr, self.len);
  }

  pub unsafe fn to_str<'a>(&self) -> &'a str {
    return std::str::from_utf8_unchecked(self.to_slice());
  }
}

impl Drop for WasmBuffer {
  fn drop(&mut self) {
    unsafe {
      if self.owned {
        let layout = Layout::array::<u8>(self.len).unwrap();
        dealloc(self.ptr, layout);
      }
    }
  }
}
