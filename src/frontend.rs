use crate::error::Result;
use crate::ffi;
use crate::printer::Printer;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

static FRONTEND: OnceCell<Mutex<Frontend>> = OnceCell::new();

pub struct Frontend {
    raw: *mut ffi::cpdb_frontend_obj_t,
}

impl Frontend {
    pub fn global() -> Result<&'static Mutex<Self>> {
        FRONTEND.get_or_try_init(|| {
            let frontend = Self::new()?;
            Ok(Mutex::new(frontend))
        })
    }

    fn new() -> Result<Self> {
        unsafe {
            let raw = ffi::cpdbGetNewFrontendObj();
            if raw.is_null() {
                Err(CpdbError::NullPointer)
            } else {
                Ok(Self { raw })
            }
        }
    }

    pub fn get_printers(&self) -> Result<Vec<Printer>> {
        unsafe {
            let mut printers_vec = Vec::new();
            let mut printers_ptr: *mut *mut ffi::cpdb_printer_obj_t = std::ptr::null_mut();
            let count = ffi::cpdbGetPrinters(
                self.raw,
                &mut printers_ptr as *mut *mut *mut ffi::cpdb_printer_obj_t,
            );

            for i in 0..count {
                let printer_ptr = *printers_ptr.offset(i as isize);
                printers_vec.push(Printer::from_raw(printer_ptr)?);
            }
            Ok(printers_vec)
        }
    }
}