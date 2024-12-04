#![doc(html_no_source)]

extern crate windows;

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::derivable_impls,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::extra_unused_lifetimes,
    clippy::useless_transmute
)]
mod bindings;
pub use bindings::*;

// windows core hack
use external_windows_core::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PCWSTR(pub *const u16);

impl AsRef<PCWSTR> for PCWSTR {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl windows_core::TypeKind for PCWSTR {
    type TypeKind = windows_core::CopyType;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PCSTR(pub *const u8);

// impl PCSTR {
//     fn test() -> Self {
//         Self(c"test message".as_ptr())
//     }
// }

impl TypeKind for PCSTR {
    type TypeKind = CopyType;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BSTR(pub *const u16);

impl TypeKind for BSTR {
    type TypeKind = CopyType;
}

extern crate self as windows_core;
