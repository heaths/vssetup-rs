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
        minwindef::{FILETIME, PULONG, ULONG},
        wtypes::BSTR,
        wtypesbase::LPCOLESTR,
    },
    um::winnt::{LCID, LPCWSTR},
};

// From https://www.nuget.org/packages/Microsoft.VisualStudio.Setup.Configuration.Native

pub const IID_ISetupInstance: IID = IID {
    data1: 0xb41463c3,
    data2: 0x8866,
    data3: 0x43b5,
    data4: [ 0xbc, 0x33, 0x2b, 0x06, 0x76, 0xf7, 0xf4, 0x2e ],
};

pub const IID_IEnumSetupInstances: IID = IID {
    data1: 0x6380bcff,
    data2: 0x41d3,
    data3: 0x4b2e,
    data4: [ 0x8b, 0x2e, 0xbf, 0x8a, 0x68, 0x10, 0xc8, 0x48 ],
};

pub const IID_ISetupConfiguration: IID = IID {
    data1: 0x42843719,
    data2: 0xdb4c,
    data3: 0x46c2,
    data4: [ 0x8e, 0x7c, 0x64, 0xf1, 0x81, 0x6e, 0xfd, 0x5b ],
};

pub const IID_ISetupConfiguration2: IID = IID {
    data1: 0x26aab78c,
    data2: 0x4a60,
    data3: 0x49d6,
    data4: [ 0xaf, 0x3b, 0x3c, 0x35, 0xbc, 0x93, 0x36, 0x5d ],
};

interfaces! {
    #[uuid("b41463c3-8866-43b5-bc33-2b0676f7f42e")]
    pub unsafe interface ISetupInstance: IUnknown {
        fn GetInstanceId(&self, pbstrInstanceId: *mut BSTR) -> HRESULT;
        fn GetInstallDate(&self, pInstallDate: *mut FILETIME) -> HRESULT;
        fn GetInstallationName(&self, pbstrInstallationName: *mut BSTR) -> HRESULT;
        fn GetInstallationPath(&self, pbstrInstallationPath: *mut BSTR) -> HRESULT;
        fn GetInstallationVersion(&self, pbstrInstallationVersion: *mut BSTR) -> HRESULT;
        fn GetDisplayName(&self, lcid: LCID, pbstrDisplayName: *mut BSTR) -> HRESULT;
        fn GetDescription(&self, lcid: LCID, pbstrDescription: *mut BSTR) -> HRESULT;
        fn ResolvePath(&self, pwszRelativePath: LPCOLESTR, pbstrAbsolutePath: *mut BSTR) -> HRESULT;
    }

    #[uuid("6380bcff-41d3-4b2e-8b2e-bf8a6810c848")]
    pub unsafe interface IEnumSetupInstances: IUnknown {
        fn Next(&self, celt: ULONG, rgelt: *mut *mut ISetupInstance, pceltFetched: PULONG) -> HRESULT;
        fn Skip(&self, celt: ULONG) -> HRESULT;
        fn Reset(&self);
        fn Clone(&self, ppenum: *mut *mut IEnumSetupInstances) -> HRESULT;
    }

    #[uuid("42843719-db4c-46c2-8e7c-64f1816efd5b")]
    pub unsafe interface ISetupConfiguration: IUnknown {
        fn EnumInstances(&self, ppEnumInstances: *mut *mut IEnumSetupInstances) -> HRESULT;
        fn GetInstanceForCurrentProcess(&self, ppInstance: *mut *mut ISetupInstance) -> HRESULT;
        fn GetInstanceForPath(&self, wzPath: LPCWSTR, ppInstance: *mut *mut ISetupInstance) -> HRESULT;
    }

    #[uuid("26AAB78C-4A60-49D6-AF3B-3C35BC93365D")]
    pub unsafe interface ISetupConfiguration2: IUnknown {
        fn EnumAllInstances(&self, ppEnumInstances: *mut *mut IEnumSetupInstances) -> HRESULT;
    }
}
