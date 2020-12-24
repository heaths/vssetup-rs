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
        SysAllocString,
        SysFreeString,
        SysStringLen,
    },
    shared::wtypes::BSTR,
};

pub struct BString {
    s: BSTR,
}

impl BString {
    pub fn new() -> BString {
        BString {
            s: ptr::null_mut(),
        }
    }

    pub fn len(&self) -> u32 {
        unsafe {
            // returns 0 if null.
            SysStringLen(self.s)
        }
    }

    fn free(&mut self) {
        if !self.s.is_null() {
            unsafe {
                SysFreeString(self.s);
            }

            self.s = ptr::null_mut();
        }
    }
}

impl Clone for BString {
    fn clone(&self) -> BString {
        BString {
            s: unsafe { SysAllocString(self.s) },
        }
    }
}

impl Default for BString {
    fn default() -> BString {
        BString::new()
    }
}

impl fmt::Display for BString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = unsafe {
            from_raw_parts(self.s, self.len() as usize)
        };

        let s = OsString::from_wide(&s[..]);
        let s = s.to_string_lossy().into_owned();

        f.write_str(&s)
    }
}

impl ops::Deref for BString {
    type Target = BSTR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl ops::DerefMut for BString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // free the current string since we'll get another.
        self.free();

        &mut self.s
    }
}

impl Drop for BString {
    fn drop(&mut self) {
        self.free();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winapi::um::oleauto::SysAllocStringLen;

    #[test]
    fn inits_empty() {
        let bstr = BString::new();

        assert_eq!(bstr.len(), 0);
        assert_eq!(bstr.to_string(), "");
    }

    #[test]
    fn default_empty() {
        let bstr: BString = Default::default();

        assert_eq!(bstr.len(), 0);
        assert_eq!(bstr.to_string(), "");
    }

    #[test]
    fn formats() {
        let buf: Vec<u16> = "hello".encode_utf16().collect();
        let bstr = unsafe {
            BString {
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

        let mut bstr = BString::new();
        f(&mut bstr as &mut BSTR);

        assert_eq!(bstr.len(), 5);
        assert_eq!(bstr.to_string(), "hello");
    }

    #[test]
    fn no_double_drop() {
        let mut bstr = BString::new();
        bstr.free();

        drop(bstr);
    }
}
