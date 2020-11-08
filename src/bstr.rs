// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{
    ffi::OsString,
    fmt,
    ops,
    os::windows::ffi::OsStringExt,
    ptr,
    slice::from_raw_parts,
};

use winapi::{
    um::oleauto::{
        SysFreeString,
        SysStringLen,
    },
    shared::wtypes::BSTR,
};

pub struct BStr {
    s: BSTR,
}

impl BStr {
    pub fn new() -> BStr {
        BStr {
            s: ptr::null_mut(),
        }
    }

    pub fn len(&self) -> u32 {
        unsafe {
            SysStringLen(self.s)
        }
    }
}

impl fmt::Display for BStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe {
            from_raw_parts(self.s, self.len() as usize)
        };

        let s = OsString::from_wide(&s[..]);
        let s = s.to_string_lossy().into_owned();

        f.write_str(&s)
    }
}

impl ops::Deref for BStr {
    type Target = BSTR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl ops::DerefMut for BStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

impl Drop for BStr {
    fn drop(&mut self) {
        unsafe {
            SysFreeString(self.s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winapi::um::oleauto::SysAllocStringLen;

    #[test]
    fn inits_empty() {
        let bstr = BStr::new();

        assert_eq!(bstr.len(), 0);
        assert_eq!(bstr.to_string(), "");
    }

    #[test]
    fn formats() {
        let buf: Vec<u16> = "hello".encode_utf16().collect();
        let bstr = unsafe {
            BStr {
                s: SysAllocStringLen(buf.as_ptr(), buf.len() as u32),
            }
        };

        assert_eq!(bstr.len(), 5);
        assert_eq!(bstr.to_string(), "hello");
    }

    #[test]
    fn derefs() {
        fn f(bstr: *mut BSTR) {
            let buf: Vec<u16> = "hello".encode_utf16().collect();
            unsafe {
                *bstr = SysAllocStringLen(buf.as_ptr(), buf.len() as u32);
            }
        }

        let mut bstr = BStr::new();
        f(&mut bstr as &mut BSTR);

        assert_eq!(bstr.len(), 5);
        assert_eq!(bstr.to_string(), "hello");
    }
}
