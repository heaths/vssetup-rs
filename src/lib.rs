// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

extern crate com;

use bindings::Windows::Win32::System::{
    Diagnostics::Debug::GetLastError,
    OleAutomation::BSTR,
    WindowsProgramming::{FileTimeToSystemTime, FILETIME, SYSTEMTIME},
};

use chrono::{DateTime, TimeZone, Utc};
use com::runtime::create_instance;

mod interfaces;
use interfaces::*;

use windows::{Error, Result, HRESULT};

const CO_E_DLLNOTFOUND: i32 = -0x7ffb_fe08; // 0x8004_01F8
const REGDB_E_CLASSNOTREG: i32 = -0x7ffb_feac; // 0x8004_0154

pub struct SetupConfiguration {
    config: Option<ISetupConfiguration>,
}

impl SetupConfiguration {
    #[cfg(windows)]
    pub fn new() -> Self {
        let config = match create_instance::<ISetupConfiguration>(&CLSID_SetupConfiguration) {
            Ok(c) => Some(c),
            Err(e) if e == CO_E_DLLNOTFOUND || e == REGDB_E_CLASSNOTREG => None,
            Err(e) => panic!("Failed to load setup configuration: {}", e),
        };

        SetupConfiguration { config }
    }

    #[cfg(not(windows))]
    pub fn new() -> Self {
        SetupConfiguration { config: None }
    }

    pub fn instances(&self) -> Option<SetupInstances> {
        if self.config.is_none() {
            return None;
        }

        if let Some(config2) = self
            .config
            .as_ref()
            .unwrap()
            .query_interface::<ISetupConfiguration2>()
        {
            let mut e = None;
            unsafe {
                if config2
                    .EnumAllInstances(&mut e as *mut _ as *mut *mut IEnumSetupInstances)
                    .is_err()
                {
                    return None;
                }

                return Some(SetupInstances { e: e.unwrap() });
            }
        }

        let config = self.config.as_ref().unwrap();
        let mut e = None;
        unsafe {
            if config
                .EnumInstances(&mut e as *mut _ as *mut *mut IEnumSetupInstances)
                .is_err()
            {
                return None;
            }

            Some(SetupInstances { e: e.unwrap() })
        }
    }
}

pub struct SetupInstances {
    e: IEnumSetupInstances,
}

impl Iterator for SetupInstances {
    type Item = SetupInstance;

    fn next(&mut self) -> Option<SetupInstance> {
        let mut instances: [Option<ISetupInstance>; 1] = [None];
        let mut fetched = 0;
        unsafe {
            if self
                .e
                .Next(
                    1,
                    instances.as_mut_ptr() as *mut *mut ISetupInstance,
                    &mut fetched,
                )
                .is_err()
                || fetched == 0
            {
                return None;
            }

            let instance = instances[0].take().unwrap();
            Some(SetupInstance { instance })
        }
    }
}

pub struct SetupInstance {
    instance: ISetupInstance,
}

impl SetupInstance {
    pub fn instance_id(&self) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            if let Err(e) = self.instance.GetInstanceId(&mut bstr).ok() {
                return Err(e);
            }
        }

        Ok(bstr.to_string())
    }

    pub fn install_date(&self) -> Result<DateTime<Utc>> {
        let mut ft = FILETIME::default();
        let mut st = SYSTEMTIME::default();
        unsafe {
            if let Err(e) = self.instance.GetInstallDate(&mut ft).ok() {
                return Err(e);
            }

            if let Err(_) = FileTimeToSystemTime(&ft, &mut st).ok() {
                let err = HRESULT(GetLastError().0);
                return Err(Error::new(
                    err,
                    "failed to convert install time to system time",
                ));
            }
        }

        let dt = Utc
            .ymd(st.wYear.into(), st.wMonth.into(), st.wDay.into())
            .and_hms_milli(
                st.wHour.into(),
                st.wMinute.into(),
                st.wSecond.into(),
                st.wMilliseconds.into(),
            );

        Ok(dt)
    }

    pub fn installation_name(&self) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            if let Err(e) = self.instance.GetInstallationName(&mut bstr).ok() {
                return Err(e);
            }
        }

        Ok(bstr.to_string())
    }

    pub fn installation_path(&self) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            if let Err(e) = self.instance.GetInstallationPath(&mut bstr).ok() {
                return Err(e);
            }
        }

        Ok(bstr.to_string())
    }

    pub fn installation_version(&self) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            if let Err(e) = self.instance.GetInstallationVersion(&mut bstr).ok() {
                return Err(e);
            }
        }

        Ok(bstr.to_string())
    }

    pub fn display_name(&self, lcid: u32) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            if let Err(e) = self.instance.GetDisplayName(lcid, &mut bstr).ok() {
                return Err(e);
            }
        }

        Ok(bstr.to_string())
    }

    pub fn description(&self, lcid: u32) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            if let Err(e) = self.instance.GetDescription(lcid, &mut bstr).ok() {
                return Err(e);
            }
        }

        Ok(bstr.to_string())
    }
}
