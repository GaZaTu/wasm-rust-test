#![allow(dead_code)]

use std::{
  alloc::{alloc, dealloc, GlobalAlloc, Layout},
  ptr::copy_nonoverlapping,
};

use lib_wasm::{_println, buffer::WasmBuffer, define_wasm_import};

extern "C" {
  fn wasm_printnr(number: i64);
  fn wasm_println(slice: u64);

  fn wasm_xtest_new(number: i32) -> u64;
  fn wasm_xtest_do_something(ptr: u64);
  fn wasm_xtest_free(ptr: u64);
}

struct XTest {
  ptr: u64,
  owned: bool,
}

impl XTest {
  unsafe fn from(ptr: u64) -> XTest {
    return XTest { ptr, owned: false };
  }

  unsafe fn new(number: i32) -> XTest {
    return XTest {
      ptr: wasm_xtest_new(number),
      owned: true,
    };
  }

  unsafe fn do_something(&self) {
    wasm_xtest_do_something(self.ptr);
  }
}

impl Drop for XTest {
  fn drop(&mut self) {
    unsafe {
      if self.owned {
        wasm_xtest_free(self.ptr);
      }
    }
  }
}

define_wasm_import!(get_string, () -> String);

#[no_mangle]
pub unsafe extern "C" fn main() -> u64 {
  _println!("xd");
  _println!("fdm? maybe? {} {} {}", 123, "pog?", get_string());

  let externef = XTest::new(9239932);
  externef.do_something();

  // printnr(0);
  // let string = "test123xd";
  // printnr(1);
  // let buffer = Buffer::from_str(string);
  // printnr(2);
  // printnr(buffer.ptr as i64);
  // printnr(buffer.len as i64);
  // printnr(buffer.to_u64() as i64);
  // println(buffer.to_u64());

  // printnr(3);
  // let rstr_ptr = get_string();
  // printnr(4);
  // let mut rstr_buf = Buffer::from_u64(rstr_ptr);
  // rstr_buf.owned = true;
  // printnr(5);

  // let rstr = rstr_buf.to_str();
  // let rstr_new_buf = Buffer::from_str(rstr);
  // printnr(6);
  // println(rstr_new_buf.to_u64());

  let mut resultstr = WasmBuffer::new_from_str("wtf this works?!");
  resultstr.owned = false;
  return resultstr.into_u64();
}
