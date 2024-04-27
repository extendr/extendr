use crate::{catch_r_error, error::Error, error::Result, robj::Robj};
use libR_sys::*;
use std::io::Read;

use super::PstreamFormat;

pub struct ReadHook {
    func: Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    data: SEXP,
}

pub trait Load {
    /// Save an object in the R data format.
    /// `version` should probably be 3.
    fn load<P: AsRef<std::path::Path>>(
        path: &P,
        format: PstreamFormat,
        hook: Option<ReadHook>,
    ) -> Result<Robj> {
        let mut reader = std::fs::File::open(path)
            .map_err(|_| Error::Other(format!("could not open file {:?}", path.as_ref())))?;
        Self::from_reader(&mut reader, format, hook)
    }

    /// Save an object in the R data format to a `Write` trait.
    /// `version` should probably be 3.
    fn from_reader<R: Read>(
        reader: &mut R,
        format: PstreamFormat,
        hook: Option<ReadHook>,
    ) -> Result<Robj> {
        unsafe extern "C" fn inchar<R: Read>(arg1: R_inpstream_t) -> ::std::os::raw::c_int {
            let reader = &mut *((*arg1).data as *mut R);
            let buf: &mut [u8] = &mut [0_u8];
            reader.read_exact(buf).map(|_| buf[0].into()).unwrap_or(-1)
        }

        unsafe extern "C" fn inbytes<R: Read>(
            arg1: R_inpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ) {
            let reader = &mut *((*arg1).data as *mut R);
            let buf = std::slice::from_raw_parts_mut(arg2 as *mut u8, arg3 as usize);
            reader.read_exact(buf).unwrap();
        }

        let read_ptr: *mut R = reader as &mut R;
        let data = unsafe { std::mem::transmute(read_ptr) };

        let (hook_func, hook_data) = if let Some(hook) = hook {
            (hook.func, hook.data)
        } else {
            (None, unsafe { R_NilValue })
        };

        // let sexp = self.get();
        // pub type R_inpstream_t = *mut R_inpstream_st;
        let mut state = R_inpstream_st {
            data,
            type_: format,
            InChar: Some(inchar::<R>),
            InBytes: Some(inbytes::<R>),
            InPersistHookFunc: hook_func,
            InPersistHookData: hook_data,
            native_encoding: [0; 64],
            nat2nat_obj: std::ptr::null_mut(),
            nat2utf8_obj: std::ptr::null_mut(),
        };

        Ok(Robj::from_sexp(catch_r_error(move || unsafe {
            R_Unserialize(&mut state as R_inpstream_t)
        })?))
    }
}

impl Load for Robj {}
