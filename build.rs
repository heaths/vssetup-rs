// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

fn main() {
    windows::build!(
        Windows::Win32::Globalization::GetUserDefaultLCID,
        Windows::Win32::System::Diagnostics::Debug::GetLastError,
        Windows::Win32::System::OleAutomation::BSTR,
        Windows::Win32::System::WindowsProgramming::{
            FILETIME,
            SYSTEMTIME,
            FileTimeToSystemTime,
        },
    )
}
