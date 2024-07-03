use std::alloc::{GlobalAlloc, Layout};

extern "C" {
  fn wasm_alloc(size: u64) -> u64;
  fn wasm_dealloc(ptr: u64);
}

struct WasmAllocator;

unsafe impl GlobalAlloc for WasmAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    return wasm_alloc(layout.size() as u64) as *mut u8;
  }

  unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
    wasm_dealloc(ptr as u64);
  }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: WasmAllocator = WasmAllocator;
