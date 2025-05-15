#![cfg_attr(not(test), no_std)]

pub use ra_fsp_sys;
pub use ra_fsp_sys::pac;

pub use ra_fsp_sys::generated::ELC_EVENT_EDMAC0_EINT;

#[cfg(feature = "mod-r_ether")]
pub mod ether;
#[cfg(feature = "mod-r_ether_phy")]
pub mod ether_phy;
#[cfg(feature = "mod-r_ioport")]
pub mod ioport;

mod macros;

#[doc(hidden)]
mod unsafe_pinned {
    use core::{cell::UnsafeCell, marker::PhantomPinned};

    pub struct UnsafePinned<T: ?Sized>(PhantomPinned, UnsafeCell<T>);

    unsafe impl<T: ?Sized + Sync> Sync for UnsafePinned<T> {}
    unsafe impl<T: ?Sized + Send> Send for UnsafePinned<T> {}

    impl<T> UnsafePinned<T> {
        pub const fn new(value: T) -> Self {
            Self(PhantomPinned, UnsafeCell::new(value))
        }
    }

    impl<T: ?Sized> UnsafePinned<T> {
        pub const fn get(&self) -> *mut T {
            self.1.get()
        }
        pub const fn raw_get(this: *const Self) -> *mut T {
            unsafe { UnsafeCell::raw_get(&raw const (*this).1) }
        }
    }
}

