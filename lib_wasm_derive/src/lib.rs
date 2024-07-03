extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn wasm_donkgen(_attr: TokenStream, input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::ItemFn);
  let fn_name = &input.sig.ident;
  let fn_argc = input.sig.inputs.len();
  let fn_rslt = match input.sig.output {
    syn::ReturnType::Type(_, _) => true,
    _ => false,
  };

  let fn_args = input.sig.inputs.iter().map(|a| {
    match a {
      syn::FnArg::Typed(a) => a.ty.to_token_stream(),
      _ => quote! {},
    }
  });
  let fn_type = match &input.sig.output {
    syn::ReturnType::Type(_, ty) => ty.to_token_stream(),
    _ => quote! {},
  };

  let wasm_fn_name = format_ident!("wasm_{}", fn_name);
  let wasm_fn_args_in = (0..fn_argc).map(|i| format_ident!("arg{}", i));
  let wasm_fn_args_out = (0..fn_argc).map(|i| format_ident!("arg{}", i));

  let wasm_fn_args_sig = quote! { #(#wasm_fn_args_in: u64),* };
  let wasm_fn_rslt_sig = if fn_rslt { quote! { -> u64 } } else { quote! { } };

  let fn_call = quote! {
    #fn_name(#(<#fn_args as lib_wasm::wasm_from::WasmFromAbi>::from_u64(#wasm_fn_args_out)),*)
  };

  let fn_call_with_return = if fn_rslt {
    quote! {
      let mut result = #fn_call;
      return <#fn_type as lib_wasm::wasm_into::WasmIntoAbi>::into_u64_unown(&mut result);
    }
  } else {
    quote! { #fn_call; }
  };

  let output = quote! {
    #input
    #[no_mangle]
    pub unsafe extern "C" fn #wasm_fn_name(#wasm_fn_args_sig) #wasm_fn_rslt_sig {
      #fn_call_with_return
    }
  };

  return TokenStream::from(output);
}
