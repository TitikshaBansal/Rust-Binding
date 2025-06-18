use crate::error::{CpdbError, Result};
use crate::ffi;
use std::ffi::CStr;

pub struct Printer {
    raw: *mut ffi::cpdb_printer_obj_t,
}

impl Printer {
    /// Create a Printer from raw pointer (unsafe)
    pub unsafe fn from_raw(raw: *mut ffi::cpdb_printer_obj_t) -> Result<Self> {
        if raw.is_null() {
            Err(CpdbError::NullPointer)
        } else {
            Ok(Self { raw })
        }
    }

    /// Get printer name
    pub fn name(&self) -> Result<String> {
        unsafe {
            let c_name = ffi::cpdbGetPrinterName(self.raw);
            if c_name.is_null() {
                Err(CpdbError::NullPointer)
            } else {
                Ok(CStr::from_ptr(c_name).to_string_lossy().into_owned())
            }
        }
    }

    /// Get printer state
    pub fn state(&self) -> Result<String> {
        unsafe {
            let c_state = ffi::cpdbGetPrinterState(self.raw);
            Ok(CStr::from_ptr(c_state).to_string_lossy().into_owned())
        }
    }

    // Add more methods as needed
}

impl Drop for Printer {
    fn drop(&mut self) {
        unsafe {
            ffi::cpdbDeletePrinterObject(self.raw);
        }
    }
}