use crate::error::{CpdbError, Result};
use crate::ffi;

pub struct PrintJob {
    raw: *mut ffi::cpdb_print_job_t,
}

impl PrintJob {
    pub fn new(
        printer_name: &str,
        options: &[(&str, &str)],
        file_path: &str,
    ) -> Result<Self> {
        // Create C-compatible options array
        let mut c_options = vec![];
        for (k, v) in options {
            c_options.push(ffi::cpdb_option_t {
                name: k.as_ptr() as *const i8,
                value: v.as_ptr() as *const i8,
            });
        }

        unsafe {
            let job = ffi::cpdbNewPrintJob(
                printer_name.as_ptr() as *const i8,
                c_options.as_ptr(),
                c_options.len() as i32,
                file_path.as_ptr() as *const i8,
            );

            if job.is_null() {
                Err(CpdbError::JobFailed("Creation failed".into()))
            } else {
                Ok(Self { raw: job })
            }
        }
    }

    pub fn submit(&self) -> Result<()> {
        unsafe {
            if ffi::cpdbSubmitPrintJob(self.raw) == 0 {
                Ok(())
            } else {
                Err(CpdbError::JobFailed("Submission failed".into()))
            }
        }
    }
}

impl Drop for PrintJob {
    fn drop(&mut self) {
        unsafe {
            ffi::cpdbCancelPrintJob(self.raw);
        }
    }
}