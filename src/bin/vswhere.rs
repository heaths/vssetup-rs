// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![cfg(windows)]

extern crate vssetup;
use vssetup::SetupConfiguration;

use windows::{
    core::Result,
    Win32::Globalization::GetUserDefaultLCID,
    Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
};

use chrono::Local;

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED)?;
    }

    let lcid = unsafe { GetUserDefaultLCID() };

    let config = SetupConfiguration::new();
    if let Some(e) = config.instances() {
        let mut first = true;
        for instance in e {
            if !first {
                println!();
            }

            println!("instanceId: {}", instance.instance_id()?);
            println!(
                "installDate: {}",
                instance.install_date()?.with_timezone(&Local)
            );
            println!("installationName: {}", instance.installation_name()?);
            println!("installationPath: {}", instance.installation_path()?);
            println!("installationVersion: {}", instance.installation_version()?);
            println!("displayName: {}", instance.display_name(lcid)?);
            println!("description: {}", instance.description(lcid)?);

            first = false;
        }
    }

    Ok(())
}
