#[macro_export]
macro_rules! define_wasm_import {
  ($name:ident, ($($arg_name:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {
    $crate::__private::paste::paste! {
      extern {
        fn [<wasm_ $name>]($($arg_name : u64),*) -> $crate::define_wasm_import!(@@return_type $($ret)?);
      }

      unsafe fn $name($($arg_name : $arg_ty),*) $(-> $ret)? {
        let raw = [<wasm_ $name>]($(
          <$arg_ty as $crate::wasm_into::WasmIntoAbi>::into_u64(&$arg_name)
        ),*);
        $crate::define_wasm_import!(@@return_value raw $($ret)?)
      }
    }
  };
  (@@return_type $ty:ty) => { u64 };
  (@@return_type ) => { () };
  (@@return_value $raw:ident $ty:ty) => { <$ty as $crate::wasm_from::WasmFromAbi>::from_u64_owned($raw) };
  (@@return_value $raw:ident ) => { () };
}
