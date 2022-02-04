//! Wrapper for R output streams.

use crate::{catch_r_error, error::Error, error::Result, robj::GetSexp};
use libR_sys::*;
use std::io::Write;

use super::PstreamFormat;

/// The hook will convert some objects into strings.
pub struct WriteHook {
    pub func: unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP,
    pub data: SEXP,
}

pub struct OutStream<W: Write> {
    r_state: R_outpstream_st,
    writer: W,
}

impl<W: Write> OutStream<W> {
    pub fn from_writer(
        writer: W,
        format: PstreamFormat,
        version: i32,
        hook: Option<WriteHook>,
    ) -> Box<OutStream<W>> {
        unsafe extern "C" fn outchar<W: Write>(arg1: R_outpstream_t, arg2: ::std::os::raw::c_int) {
            let writer = &mut *((*arg1).data as *mut W);
            let b = [arg2 as u8];
            writer.write_all(&b).unwrap();
        }

        unsafe extern "C" fn outbytes<W: Write>(
            arg1: R_outpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ) {
            let writer = &mut *((*arg1).data as *mut W);
            let b = std::slice::from_raw_parts(arg2 as *mut u8, arg3 as usize);
            writer.write_all(b).unwrap();
        }

        {
            let (hook_fn, hook_data) = if let Some(WriteHook { func, data }) = hook {
                (Some(func), data)
            } else {
                unsafe { (None, R_NilValue) }
            };

            let r_state = libR_sys::R_outpstream_st {
                data: std::ptr::null_mut(),
                type_: format as R_pstream_format_t,
                version,
                OutChar: Some(outchar::<W>),
                OutBytes: Some(outbytes::<W>),
                OutPersistHookFunc: hook_fn,
                OutPersistHookData: hook_data,
            };
            let mut os = Box::new(OutStream { r_state, writer });
            os.r_state.data = &mut os.writer as *mut W as R_pstream_data_t;
            os
        }
    }
}

pub trait Save: GetSexp {
    /// Save an object in the R data format.
    /// `version` should probably be 3.
    fn save<P: AsRef<std::path::Path>>(
        &self,
        path: &P,
        format: PstreamFormat,
        version: i32,
        hook: Option<WriteHook>,
    ) -> Result<()> {
        let mut writer = std::fs::File::create(path.as_ref())
            .map_err(|_| Error::Other(format!("could not create file {:?}", path.as_ref())))?;
        self.to_writer(&mut writer, format, version, hook)
    }

    /// Save an object in the R data format to a `Write` trait.
    /// `version` should probably be 3.
    fn to_writer<W: Write>(
        &self,
        writer: &mut W,
        format: PstreamFormat,
        version: i32,
        hook: Option<WriteHook>,
    ) -> Result<()> {
        let mut os = OutStream::from_writer(writer, format, version, hook);

        let stream = &mut os.r_state as R_outpstream_t;
        let sexp = unsafe { self.get() };
        if !(2..=3).contains(&version) {
            return Err(Error::Other(format!(
                "version must be 2 or 3, got {:?}",
                version
            )));
        }

        catch_r_error(move || unsafe {
            R_Serialize(sexp, stream);
            R_NilValue
        })?;
        Ok(())
    }
}

impl<R: GetSexp> Save for R {}
