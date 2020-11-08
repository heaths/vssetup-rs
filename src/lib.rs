// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

extern crate com;
extern crate winapi;

mod bstr;
use bstr::BStr;

use com::runtime::create_instance;
use com::sys::{
    FAILED,
    HRESULT,
};

mod interfaces;
use interfaces::{
    CLSID_SetupConfiguration,
    IEnumSetupInstances,
    ISetupConfiguration,
    ISetupConfiguration2,
    ISetupInstance,
};

const CO_E_DLLNOTFOUND: HRESULT = -0x7ffb_fe08;
const REGDB_E_CLASSNOTREG: HRESULT = -0x7ffb_feac;

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

        SetupConfiguration {
            config,
        }
    }

    #[cfg(not(windows))]
    pub fn new() -> Self {
        SetupConfiguration {
            config: None,
        }
    }

    pub fn instances(&self) -> Option<SetupInstances> {
        if self.config.is_none() {
            return None;
        }

        if let Some(config2) = self.config.as_ref().unwrap()
            .query_interface::<ISetupConfiguration2>() {
                let mut e = None;
                unsafe {
                    if FAILED(config2.EnumAllInstances(&mut e as *mut _ as *mut *mut IEnumSetupInstances)) {
                        return None;
                    }

                    return Some(
                        SetupInstances {
                            e: e.unwrap(),
                        }
                    );
                }
        }

        let config = self.config.as_ref().unwrap();
        let mut e = None;
        unsafe {
            if FAILED(config.EnumInstances(&mut e as *mut _ as *mut *mut IEnumSetupInstances)) {
                return None;
            }

            Some(
                SetupInstances {
                    e: e.unwrap(),
                }
            )
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
            if FAILED(self.e.Next(1, instances.as_mut_ptr() as *mut *mut ISetupInstance, &mut fetched)) || fetched == 0 {
                return None;
            }

            let instance = instances[0].take().unwrap();
            Some(
                SetupInstance {
                    instance,
                }
            )
        }
    }
}

pub struct SetupInstance {
    instance: ISetupInstance,
}

impl SetupInstance {
    pub fn instance_id(&self) -> String {
        let mut bstr = BStr::new();
        unsafe {
            self.instance.GetInstanceId(&mut (*bstr));
        }

        bstr.to_string()
    }

    pub fn installation_name(&self) -> String {
        let mut bstr = BStr::new();
        unsafe {
            self.instance.GetInstallationName(&mut (*bstr));
        }

        bstr.to_string()
    }

    pub fn installation_path(&self) -> String {
        let mut bstr = BStr::new();
        unsafe {
            self.instance.GetInstallationPath(&mut (*bstr));
        }

        bstr.to_string()
    }

    pub fn installation_version(&self) -> String {
        let mut bstr = BStr::new();
        unsafe {
            self.instance.GetInstallationVersion(&mut (*bstr));
        }

        bstr.to_string()
    }

    pub fn display_name(&self, lcid: u32) -> String {
        let mut bstr = BStr::new();
        unsafe {
            self.instance.GetDisplayName(lcid, &mut (*bstr));
        }

        bstr.to_string()
    }

    pub fn description(&self, lcid: u32) -> String {
        let mut bstr = BStr::new();
        unsafe {
            self.instance.GetDescription(lcid, &mut (*bstr));
        }

        bstr.to_string()
    }
}
