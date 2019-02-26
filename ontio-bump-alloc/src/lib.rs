#![no_std]
use cfg_if::cfg_if;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr;

const PAGE_SIZE: usize = 65536;

struct Inner {
    pos: *mut u8,
    size: usize,
}

pub struct BumpAlloc {
    inner: RefCell<Inner>,
}

//contract execution is single thread
unsafe impl Sync for BumpAlloc {}

impl BumpAlloc {
    pub const fn new() -> BumpAlloc {
        BumpAlloc { inner: RefCell::new(Inner { pos: ptr::null_mut(), size: 0 }) }
    }
}

fn align_to(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut inner = self.inner.borrow_mut();

        let need_bytes = align_to(layout.size(), layout.align());
        let pos = align_to(inner.pos as usize, layout.align());
        if pos + need_bytes > inner.size {
            let need_page = (pos + need_bytes - inner.size + PAGE_SIZE - 1) / PAGE_SIZE;
            match alloc_pages(need_page) {
                Some(p) => {
                    if inner.pos.is_null() {
                        inner.pos = p;
                    }
                    inner.size = (p as usize) + need_page * PAGE_SIZE;
                }
                None => return ptr::null_mut(),
            }
        }

        let pos = align_to(inner.pos as usize, layout.align());
        inner.pos = (pos + need_bytes) as *mut u8;

        pos as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

cfg_if! {
if #[cfg(target_arch = "wasm32")] {
    mod alloc_impl {
        use core::arch::wasm32;
        use super::PAGE_SIZE;

        pub(crate) unsafe fn alloc_pages(num_page: usize) -> Option<*mut u8> {
            let ptr = wasm32::memory_grow(0, num_page);
            if ptr != usize::max_value() {
                let ptr = (ptr * PAGE_SIZE) as *mut u8;
                Some(ptr)
            } else {
                None
            }
        }
    }
    use alloc_impl::alloc_pages;
} else {
    mod imp_noop {
        pub(crate) unsafe fn alloc_pages(_num_page: usize) -> Option<*mut u8> {
            None
        }
    }
    use imp_noop::alloc_pages;
}
}
