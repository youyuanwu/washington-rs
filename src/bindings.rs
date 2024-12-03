// Bindings generated by `windows-bindgen` 0.58.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod States {
    pub mod Washington {
        windows_core::imp::define_interface!(
            IWashington,
            IWashington_Vtbl,
            0xa7c97d53_cf24_4453_bd17_55a48e1d0510
        );
        impl core::ops::Deref for IWashington {
            type Target = windows_core::IUnknown;
            fn deref(&self) -> &Self::Target {
                unsafe { core::mem::transmute(self) }
            }
        }
        windows_core::imp::interface_hierarchy!(IWashington, windows_core::IUnknown);
        impl IWashington {
            pub unsafe fn Load(&self) -> windows_core::Result<()> {
                (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self))
                    .ok()
            }
            pub unsafe fn LoadFrom<P0>(&self, path: P0) -> windows_core::Result<()>
            where
                P0: windows_core::Param<windows_core::PCWSTR>,
            {
                (windows_core::Interface::vtable(self).LoadFrom)(
                    windows_core::Interface::as_raw(self),
                    path.param().abi(),
                )
                .ok()
            }
        }
        #[repr(C)]
        pub struct IWashington_Vtbl {
            pub base__: windows_core::IUnknown_Vtbl,
            pub Load: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            pub LoadFrom: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                windows_core::PCWSTR,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IWashington2,
            IWashington2_Vtbl,
            0xe27af699_bc37_47b8_ad97_1c6720389efd
        );
        impl core::ops::Deref for IWashington2 {
            type Target = IWashington;
            fn deref(&self) -> &Self::Target {
                unsafe { core::mem::transmute(self) }
            }
        }
        windows_core::imp::interface_hierarchy!(IWashington2, windows_core::IUnknown, IWashington);
        impl IWashington2 {
            pub unsafe fn Load2(&self) -> windows_core::Result<()> {
                (windows_core::Interface::vtable(self).Load2)(windows_core::Interface::as_raw(self))
                    .ok()
            }
        }
        #[repr(C)]
        pub struct IWashington2_Vtbl {
            pub base__: IWashington_Vtbl,
            pub Load2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
        }
        pub trait IWashington_Impl: Sized {
            fn Load(&self) -> windows_core::Result<()>;
            fn LoadFrom(&self, path: &windows_core::PCWSTR) -> windows_core::Result<()>;
        }
        impl windows_core::RuntimeName for IWashington {}
        impl IWashington_Vtbl {
            pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(
            ) -> IWashington_Vtbl
            where
                Identity: IWashington_Impl,
            {
                unsafe extern "system" fn Load<
                    Identity: windows_core::IUnknownImpl,
                    const OFFSET: isize,
                >(
                    this: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT
                where
                    Identity: IWashington_Impl,
                {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IWashington_Impl::Load(this).into()
                }
                unsafe extern "system" fn LoadFrom<
                    Identity: windows_core::IUnknownImpl,
                    const OFFSET: isize,
                >(
                    this: *mut core::ffi::c_void,
                    path: windows_core::PCWSTR,
                ) -> windows_core::HRESULT
                where
                    Identity: IWashington_Impl,
                {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IWashington_Impl::LoadFrom(this, core::mem::transmute(&path)).into()
                }
                Self {
                    base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
                    Load: Load::<Identity, OFFSET>,
                    LoadFrom: LoadFrom::<Identity, OFFSET>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IWashington as windows_core::Interface>::IID
            }
        }
        pub trait IWashington2_Impl: Sized + IWashington_Impl {
            fn Load2(&self) -> windows_core::Result<()>;
        }
        impl windows_core::RuntimeName for IWashington2 {}
        impl IWashington2_Vtbl {
            pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(
            ) -> IWashington2_Vtbl
            where
                Identity: IWashington2_Impl,
            {
                unsafe extern "system" fn Load2<
                    Identity: windows_core::IUnknownImpl,
                    const OFFSET: isize,
                >(
                    this: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT
                where
                    Identity: IWashington2_Impl,
                {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    IWashington2_Impl::Load2(this).into()
                }
                Self {
                    base__: IWashington_Vtbl::new::<Identity, OFFSET>(),
                    Load2: Load2::<Identity, OFFSET>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IWashington2 as windows_core::Interface>::IID
                    || iid == &<IWashington as windows_core::Interface>::IID
            }
        }
    }
    windows_core::imp::define_interface!(
        IState,
        IState_Vtbl,
        0x2f0cf45d_04a9_43cc_bc50_ebfe429fcecf
    );
    impl core::ops::Deref for IState {
        type Target = windows_core::IUnknown;
        fn deref(&self) -> &Self::Target {
            unsafe { core::mem::transmute(self) }
        }
    }
    windows_core::imp::interface_hierarchy!(IState, windows_core::IUnknown);
    impl IState {
        pub unsafe fn GetFlower(&self) -> windows_core::Result<windows_core::BSTR> {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlower)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[repr(C)]
    pub struct IState_Vtbl {
        pub base__: windows_core::IUnknown_Vtbl,
        pub GetFlower: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::mem::MaybeUninit<windows_core::BSTR>,
        ) -> windows_core::HRESULT,
    }
    pub trait IState_Impl: Sized {
        fn GetFlower(&self) -> windows_core::Result<windows_core::BSTR>;
    }
    impl windows_core::RuntimeName for IState {}
    impl IState_Vtbl {
        pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IState_Vtbl
        where
            Identity: IState_Impl,
        {
            unsafe extern "system" fn GetFlower<
                Identity: windows_core::IUnknownImpl,
                const OFFSET: isize,
            >(
                this: *mut core::ffi::c_void,
                flower: *mut core::mem::MaybeUninit<windows_core::BSTR>,
            ) -> windows_core::HRESULT
            where
                Identity: IState_Impl,
            {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IState_Impl::GetFlower(this) {
                    Ok(ok__) => {
                        flower.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
            Self {
                base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
                GetFlower: GetFlower::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IState as windows_core::Interface>::IID
        }
    }
}
