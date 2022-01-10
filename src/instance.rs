// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use windows::Win32::{
    Foundation::{BSTR, FILETIME, SYSTEMTIME},
    System::Time::FileTimeToSystemTime,
};

use chrono::{DateTime, TimeZone, Utc};

use crate::errors::*;
use crate::interfaces::ISetupInstance;

pub struct SetupInstance {
    pub(crate) instance: ISetupInstance,
}

impl SetupInstance {
    pub fn instance_id(&self) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            self.instance.GetInstanceId(&mut bstr).ok()?;
        }

        Ok(bstr.to_string())
    }

    pub fn install_date(&self) -> Result<DateTime<Utc>> {
        let mut ft = FILETIME::default();
        let mut st = SYSTEMTIME::default();
        unsafe {
            self.instance.GetInstallDate(&mut ft).ok()?;
            FileTimeToSystemTime(&ft, &mut st).ok()?;
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
            self.instance.GetInstallationName(&mut bstr).ok()?;
        }

        Ok(bstr.to_string())
    }

    pub fn installation_path(&self) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            self.instance.GetInstallationPath(&mut bstr).ok()?;
        }

        Ok(bstr.to_string())
    }

    pub fn installation_version(&self) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            self.instance.GetInstallationVersion(&mut bstr).ok()?;
        }

        Ok(bstr.to_string())
    }

    pub fn display_name(&self, lcid: u32) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            self.instance.GetDisplayName(lcid, &mut bstr).ok()?;
        }

        Ok(bstr.to_string())
    }

    pub fn description(&self, lcid: u32) -> Result<String> {
        let mut bstr = BSTR::default();
        unsafe {
            self.instance.GetDescription(lcid, &mut bstr).ok()?;
        }

        Ok(bstr.to_string())
    }
}
