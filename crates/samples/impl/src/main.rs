use std::ffi::CString;

use washington_rs::{Microsoft::States::*, PCSTR, PCWSTR};
use windows_core::*;

#[derive(Debug)]
#[implement(IState)]
struct Atlantis {
    flower: CString,
}

#[allow(non_snake_case)]
impl IState_Impl for Atlantis_Impl {
    fn GetFlower(&self) -> Result<PCSTR> {
        Ok(PCSTR(self.flower.as_ptr() as *const u8))
    }

    fn GetData2(&self) -> PCWSTR {
        PCWSTR(std::ptr::null())
    }
}

fn main() -> windows_core::Result<()> {
    let state: IState = Atlantis {
        flower: CString::new("Red Algae").unwrap(),
    }
    .into();

    let ptr = unsafe { state.GetFlower() }?;
    let flower = unsafe { std::ffi::CStr::from_ptr(ptr.0 as *const i8) };
    println!("{:?}", flower);
    Ok(())
}
