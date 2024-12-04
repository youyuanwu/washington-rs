use std::ffi::CString;

use washington_rs::Microsoft::States::*;
use windows::core::*;

#[derive(Debug)]
#[implement(IState)]
struct Atlantis {
    flower: CString,
}

#[allow(non_snake_case)]
impl IState_Impl for Atlantis_Impl {
    fn GetFlower(&self) -> Result<windows_core::PCSTR> {
        Ok(PCSTR(self.flower.as_ptr() as *const u8))
    }

    fn GetData2(&self) -> windows_core::PCWSTR {
        PCWSTR::null()
    }
}

fn main() -> windows::core::Result<()> {
    let state: IState = Atlantis {
        flower: CString::new("Red Algae").unwrap(),
    }
    .into();

    let ptr = unsafe { state.GetFlower() }?;
    let flower = CString::new(unsafe { ptr.as_bytes() }).unwrap();
    println!("{:?}", flower);
    Ok(())
}
