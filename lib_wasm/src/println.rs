#![allow(dead_code)]

extern "C" {
  fn wasm_println(string: u64);
}

#[macro_export]
macro_rules! _println {
  ($($arg:tt)*) => {{
    let string = format!($($arg)*);
    let buffer = WasmBuffer::from_str(string.as_str());
    wasm_println(buffer.into_u64());
  }};
}
