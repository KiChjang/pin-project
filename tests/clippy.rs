#![warn(rust_2018_idioms, single_use_lifetimes)]
#![warn(unused, future_incompatible)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use pin_project::{pin_project, pinned_drop, UnsafeUnpin};
use std::pin::Pin;

#[pin_project]
pub struct StructDefault<T, U> {
    #[pin]
    pub pinned: T,
    pub unpinned: U,
}

#[pin_project(UnsafeUnpin)]
pub struct StructUnsafeUnpin<T, U> {
    #[pin]
    pub pinned: T,
    pub unpinned: U,
}

unsafe impl<T: Unpin, U> UnsafeUnpin for StructUnsafeUnpin<T, U> {}

#[pin_project(PinnedDrop)]
pub struct StructPinnedDrop<T, U> {
    #[pin]
    pub pinned: T,
    pub unpinned: U,
}

#[pinned_drop]
impl<T, U> PinnedDrop for StructPinnedDrop<T, U> {
    #[allow(clippy::needless_pass_by_value)] // FIXME: https://github.com/rust-lang/rust-clippy/issues/3031?
    fn drop(self: Pin<&mut Self>) {}
}

#[pin_project(Replace)]
pub struct StructReplace<T, U> {
    #[pin]
    pub pinned: T,
    pub unpinned: U,
}

#[pin_project]
pub struct StructMutMut<'a, T, U> {
    #[pin]
    pub pinned: &'a mut T,
    pub unpinned: &'a mut U,
}

#[pin_project]
pub enum EnumDefault<T, U> {
    Variant {
        #[pin]
        pinned: T,
        unpinned: U,
    },
}

#[pin_project(UnsafeUnpin)]
pub enum EnumUnsafeUnpin<T, U> {
    Variant {
        #[pin]
        pinned: T,
        unpinned: U,
    },
}

unsafe impl<T: Unpin, U> UnsafeUnpin for EnumUnsafeUnpin<T, U> {}

#[pin_project(PinnedDrop)]
pub enum EnumPinnedDrop<T, U> {
    Variant {
        #[pin]
        pinned: T,
        unpinned: U,
    },
}

#[pinned_drop]
impl<T, U> PinnedDrop for EnumPinnedDrop<T, U> {
    #[allow(clippy::needless_pass_by_value)] // FIXME: https://github.com/rust-lang/rust-clippy/issues/3031?
    fn drop(self: Pin<&mut Self>) {}
}

#[pin_project(Replace)]
pub enum EnumReplace<T, U> {
    Variant {
        #[pin]
        pinned: T,
        unpinned: U,
    },
}

#[pin_project]
pub enum EnumMutMut<'a, T, U> {
    Variant {
        #[pin]
        pinned: &'a mut T,
        unpinned: &'a mut U,
    },
}

#[allow(clippy::missing_const_for_fn)]
#[test]
fn test() {}
