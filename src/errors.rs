// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::error;
use std::fmt;
use windows::core;

pub type Result<T> = std::result::Result<T, SetupConfigurationError>;

const E_NOTIMPL: u32 = 0x8000_4001;

#[derive(Debug, PartialEq)]
pub enum SetupConfigurationError {
    NotInstalled,
    NotImplemented,
    COM { err: core::Error },
}

impl fmt::Display for SetupConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SetupConfigurationError::NotInstalled => {
                write!(f, "setup configuration module is not installed")
            }
            SetupConfigurationError::NotImplemented => {
                write!(f, "not implemented")
            }
            SetupConfigurationError::COM { err } => err.fmt(f),
        }
    }
}

impl error::Error for SetupConfigurationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SetupConfigurationError::NotInstalled => None,
            SetupConfigurationError::NotImplemented => None,
            SetupConfigurationError::COM { ref err } => Some(err),
        }
    }
}

impl From<core::Error> for SetupConfigurationError {
    fn from(err: core::Error) -> Self {
        match err.code().0 {
            E_NOTIMPL => SetupConfigurationError::NotImplemented,
            _ => SetupConfigurationError::COM { err },
        }
    }
}

impl From<com::sys::HRESULT> for SetupConfigurationError {
    fn from(hr: com::sys::HRESULT) -> Self {
        let _hr = core::HRESULT(hr as u32);
        match _hr.0 {
            E_NOTIMPL => SetupConfigurationError::NotImplemented,
            _ => SetupConfigurationError::COM {
                err: core::Error::from(_hr),
            },
        }
    }
}

impl From<core::HRESULT> for SetupConfigurationError {
    fn from(hr: core::HRESULT) -> Self {
        match hr.0 {
            E_NOTIMPL => SetupConfigurationError::NotImplemented,
            _ => SetupConfigurationError::COM {
                err: core::Error::from(hr),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notinstalled_fmt() {
        assert_eq!(
            "setup configuration module is not installed",
            format!("{}", SetupConfigurationError::NotInstalled)
        )
    }

    #[test]
    fn notimplemented_fmt() {
        assert_eq!(
            "not implemented",
            format!("{}", SetupConfigurationError::NotImplemented)
        )
    }

    #[test]
    #[cfg(windows)]
    fn com_fmt() {
        let err: SetupConfigurationError = core::HRESULT(0x80070490).into();
        assert_ne!(0, format!("{}", err).len())
    }

    #[test]
    fn from_error_notimplemented() {
        let hr = core::HRESULT(E_NOTIMPL);
        let err: SetupConfigurationError = core::Error::from(hr).into();
        assert_eq!(SetupConfigurationError::NotImplemented, err)
    }

    #[test]
    fn from_com_sys_hresult_notimplemented() {
        let hr: com::sys::HRESULT = E_NOTIMPL as i32;
        let err: SetupConfigurationError = hr.into();
        assert_eq!(SetupConfigurationError::NotImplemented, err)
    }

    #[test]
    fn from_core_hresult_notimplemented() {
        let hr = core::HRESULT(E_NOTIMPL);
        let err: SetupConfigurationError = hr.into();
        assert_eq!(SetupConfigurationError::NotImplemented, err)
    }
}
