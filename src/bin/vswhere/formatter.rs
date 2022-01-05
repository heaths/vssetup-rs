// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![cfg(windows)]

extern crate vssetup;
use vssetup::{Result, SetupInstances};

use chrono::Local;
use windows::Win32::Globalization::GetUserDefaultLCID;

pub fn print_instances(instances: SetupInstances) -> Result<()> {
    let lcid = unsafe { GetUserDefaultLCID() };

    let mut first = true;
    for instance in instances {
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

    Ok(())
}
