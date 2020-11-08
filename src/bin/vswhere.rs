// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![cfg(windows)]

extern crate vssetup;
use vssetup::SetupConfiguration;

use com::runtime::init_runtime;

fn main() {
    init_runtime().expect("Failed to initialize COM");

    let config = SetupConfiguration::new();
    if let Some(e) = config.instances() {
        let mut first = true;
        for instance in e {
            if !first {
                println!();
            }

            println!("InstanceId: {}", instance.instance_id());
            println!("InstallationName: {}", instance.installation_name());
            println!("InstallationPath: {}", instance.installation_path());
            println!("InstallationVersion: {}", instance.installation_version());

            first = false;
        }
    }
}
