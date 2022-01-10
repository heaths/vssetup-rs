// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

extern crate com;
pub use com::runtime::ApartmentType;
use com::runtime::{create_instance, ApartmentRuntime};

mod errors;
pub use errors::*;

mod instance;
pub use instance::*;

mod interfaces;
use interfaces::{
    CLSID_SetupConfiguration, IEnumSetupInstances, ISetupConfiguration, ISetupConfiguration2,
    ISetupInstance,
};

use std::mem;

const CO_E_DLLNOTFOUND: i32 = -0x7ffb_fe08; // 0x8004_01F8
const REGDB_E_CLASSNOTREG: i32 = -0x7ffb_feac; // 0x8004_0154

pub struct SetupConfiguration {
    apartment: Option<ApartmentRuntime>,
    config: Option<ISetupConfiguration>,
}

#[cfg(windows)]
impl SetupConfiguration {
    pub fn new() -> Result<Self> {
        let config = Self::create_instance()?;

        Ok(SetupConfiguration {
            apartment: None,
            config: Some(config),
        })
    }

    pub fn with_apartment(apartment_type: ApartmentType) -> Result<Self> {
        let apartment = ApartmentRuntime::new(apartment_type)?;
        let config = Self::create_instance()?;

        Ok(SetupConfiguration {
            apartment: Some(apartment),
            config: Some(config),
        })
    }

    fn create_instance() -> Result<ISetupConfiguration> {
        let config = create_instance::<ISetupConfiguration>(&CLSID_SetupConfiguration).map_err(
            |e| match e {
                CO_E_DLLNOTFOUND | REGDB_E_CLASSNOTREG => SetupConfigurationError::NotInstalled,
                _ => e.into(),
            },
        )?;

        Ok(config)
    }
}

#[cfg(not(windows))]
impl SetupConfiguration {
    pub fn new() -> Result<Self> {
        Ok(SetupConfiguration {
            apartment: None,
            config: None,
        })
    }

    pub fn with_apartment() -> Result<Self> {
        SetupConfiguration::new()
    }
}

impl Drop for SetupConfiguration {
    fn drop(&mut self) {
        // Make sure the COM object is released before the apartment freed.
        if let Some(config) = &self.config {
            mem::drop(config);
            self.config = None;
        }

        if let Some(apartment) = &self.apartment {
            mem::drop(apartment);
            self.apartment = None;
        }
    }
}

impl SetupConfiguration {
    pub fn instances(&self, all: bool) -> Result<Option<SetupInstances>> {
        if self.config.is_none() {
            return Ok(None);
        }

        let config = self.config.as_ref().unwrap();
        let mut e = None;
        if all {
            let config2 = config
                .query_interface::<ISetupConfiguration2>()
                .ok_or(SetupConfigurationError::NotImplemented)?;

            unsafe {
                config2
                    .EnumAllInstances(&mut e as *mut _ as *mut *mut IEnumSetupInstances)
                    .ok()?;
            }
        } else {
            unsafe {
                config
                    .EnumInstances(&mut e as *mut _ as *mut *mut IEnumSetupInstances)
                    .ok()?;
            }
        }

        Ok(Some(SetupInstances { e: e.unwrap() }))
    }
}

pub struct SetupInstances {
    e: IEnumSetupInstances,
}

impl Iterator for SetupInstances {
    type Item = SetupInstance;

    #[cfg(windows)]
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

    #[cfg(not(windows))]
    fn next(&mut self) -> Option<SetupInstance> {
        None
    }
}
