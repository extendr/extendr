//! Wrapper for R output streams.

use crate::test;
use crate::{catch_r_error, error::Error, error::Result, robj::GetSexp};
use libR_sys::*;
use std::io::Write;

/// The hook will convert some objects into strings.
pub struct Hook {
    pub func: unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP,
    pub data: SEXP,
}

pub struct OutStream<W: Write> {
    r_state: R_outpstream_st,
    writer: W,
}

pub enum PstreamFormat {
    AnyFormat = 0,
    AsciiFormat = 1,
    BinaryFormat = 2,
    XdrFormat = 3,
    AsciihexFormat = 4,
}

impl<W: Write> OutStream<W> {
    pub fn from_writer(
        writer: W,
        format: PstreamFormat,
        version: i32,
        hook: Option<Hook>,
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
            let (hook_fn, hook_data) = if let Some(Hook{func, data}) = hook {
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

pub trait Serialize {
    fn save_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: &P,
        format: PstreamFormat,
        version: i32,
        hook: Option<Hook>,
    ) -> Result<()>;
    fn save_to_bytes(&self, format: PstreamFormat, version: i32, hook: Option<Hook>) -> Result<Vec<u8>>;
}

fn save(stream: R_outpstream_t, sexp: SEXP, version: i32) -> Result<()> {
    if version < 2 || version > 3 {
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

impl<R: GetSexp> Serialize for R {
    fn save_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: &P,
        format: PstreamFormat,
        version: i32,
        hook: Option<Hook>,
    ) -> Result<()> {
        let writer = std::fs::File::create(path.as_ref())
            .map_err(|_| Error::Other(format!("could not create file {:?}", path.as_ref())))?;
        let mut os = OutStream::from_writer(writer, format, version, hook);

        let stream = &mut os.r_state as R_outpstream_t;
        let sexp = unsafe { self.get() };
        save(stream, sexp, version)?;

        Ok(())
    }

    fn save_to_bytes(&self, format: PstreamFormat, version: i32, hook: Option<Hook>) -> Result<Vec<u8>> {
        let writer = Vec::new();
        let mut os = OutStream::from_writer(writer, format, version, hook);

        let stream = &mut os.r_state as R_outpstream_t;
        let sexp = unsafe { self.get() };
        save(stream, sexp, version)?;

        Ok(os.writer)
    }
}

#[test]
fn test() {
    use crate as extendr_api;
    use extendr_api::{Result, Robj};
    test! {
        let v = Robj::from(1).save_to_bytes(PstreamFormat::AsciiFormat, 3, None)?;
        let s = std::str::from_utf8(&v).unwrap();
        // println!("{}", s);
        assert!(s.starts_with("A\n"));
        assert!(s.contains("\nUTF-8\n"));

        let v = Robj::from(1).save_to_bytes(PstreamFormat::BinaryFormat, 3, None)?;
        assert!(v[0] == b'B');

        // let path : std::path::PathBuf = "/tmp/1".into();
        // Robj::from(1).save_to_file(&path, PstreamFormat::AsciiFormat, 3, None)?;
        // let s = std::fs::read(path).unwrap();
        // assert!(s.starts_with(b"A\n"));
    }
}
