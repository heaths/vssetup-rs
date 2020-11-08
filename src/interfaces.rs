// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use com::{
    interfaces,
    interfaces::iunknown::IUnknown,
    sys::{HRESULT, IID},
};

use winapi::{
    shared::{
        minwindef::{FILETIME, ULONG},
        wtypes::BSTR,
        wtypesbase::LPCOLESTR,
    },
    um::winnt::{LCID, LPCWSTR},
};

// From https://www.nuget.org/packages/Microsoft.VisualStudio.Setup.Configuration.Native

pub const CLSID_SetupConfiguration: IID = IID {
    data1: 0x177f0c4a,
    data2: 0x1cd3,
    data3: 0x4de7,
    data4: [ 0xa3, 0x2c, 0x71, 0xdb, 0xbb, 0x9f, 0xa3, 0x6d ],
};

interfaces! {
    #[uuid("b41463c3-8866-43b5-bc33-2b0676f7f42e")]
    pub unsafe interface ISetupInstance: IUnknown {
        pub fn GetInstanceId(&self, pbstrInstanceId: *mut BSTR) -> HRESULT;
        fn GetInstallDate(&self, pInstallDate: *mut FILETIME) -> HRESULT;
        pub fn GetInstallationName(&self, pbstrInstallationName: *mut BSTR) -> HRESULT;
        pub fn GetInstallationPath(&self, pbstrInstallationPath: *mut BSTR) -> HRESULT;
        pub fn GetInstallationVersion(&self, pbstrInstallationVersion: *mut BSTR) -> HRESULT;
        pub fn GetDisplayName(&self, lcid: LCID, pbstrDisplayName: *mut BSTR) -> HRESULT;
        pub fn GetDescription(&self, lcid: LCID, pbstrDescription: *mut BSTR) -> HRESULT;
        fn ResolvePath(&self, pwszRelativePath: LPCOLESTR, pbstrAbsolutePath: *mut BSTR) -> HRESULT;
    }

    #[uuid("6380bcff-41d3-4b2e-8b2e-bf8a6810c848")]
    pub unsafe interface IEnumSetupInstances: IUnknown {
        pub fn Next(&self, celt: ULONG, rgelt: *mut *mut ISetupInstance, pceltFetched: *mut ULONG) -> HRESULT;
        fn Skip(&self, celt: ULONG) -> HRESULT;
        fn Reset(&self);
        fn Clone(&self, ppenum: *mut *mut IEnumSetupInstances) -> HRESULT;
    }

    #[uuid("42843719-db4c-46c2-8e7c-64f1816efd5b")]
    pub unsafe interface ISetupConfiguration: IUnknown {
        pub fn EnumInstances(&self, ppEnumInstances: *mut *mut IEnumSetupInstances) -> HRESULT;
        fn GetInstanceForCurrentProcess(&self, ppInstance: *mut *mut ISetupInstance) -> HRESULT;
        fn GetInstanceForPath(&self, wzPath: LPCWSTR, ppInstance: *mut *mut ISetupInstance) -> HRESULT;
    }

    #[uuid("26AAB78C-4A60-49D6-AF3B-3C35BC93365D")]
    pub unsafe interface ISetupConfiguration2: IUnknown {
        pub fn EnumAllInstances(&self, ppEnumInstances: *mut *mut IEnumSetupInstances) -> HRESULT;
    }
}
