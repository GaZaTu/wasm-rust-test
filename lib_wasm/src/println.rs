#![allow(dead_code)]

extern "C" {
  fn wasm_println(string: u64);
}

#[macro_export]
macro_rules! _println {
  ($($arg:tt)*) => {{
    use $crate::wasm_into::WasmIntoAbi as _;
    let string = format!($($arg)*);
    let buffer = $crate::buffer::WasmBuffer::from_str(string.as_str());
    wasm_println(buffer.into_u64());
  }};
}
