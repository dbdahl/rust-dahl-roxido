#![allow(dead_code)]

// Help: https://docs.rs/libR-sys, https://github.com/hadley/r-internals
use libR_sys::*;

pub use libR_sys;
pub use libR_sys::SEXP;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::os::raw::{c_char, c_double, c_int};

pub struct R;

impl R {
    // Use to seed a RNG based on R's RNG state.
    pub fn random_bytes_from_r<const LENGTH: usize>() -> [u8; LENGTH] {
        unsafe {
            let result = Rf_install(b"sample.int\0".as_ptr() as *const c_char)
                .protect()
                .call2(
                    R::integer((u8::MAX as c_int) + 1).protect(),
                    R::integer(LENGTH as c_int).protect(),
                )
                .protect();
            let slice = result.as_integer_slice();
            let mut bytes: [u8; LENGTH] = [0; LENGTH];
            bytes
                .iter_mut()
                .zip(slice)
                .for_each(|(b, s)| *b = (*s - 1) as u8);
            R::unprotect(4);
            bytes
        }
    }
    pub unsafe fn print(x: &str) {
        Rprintf(
            b"%.*s\0".as_ptr() as *const c_char,
            x.len(),
            x.as_ptr() as *const c_char,
        );
    }
    pub fn unprotect(i: c_int) {
        unsafe { Rf_unprotect(i) }
    }

    pub fn integer(x: c_int) -> SEXP {
        unsafe { Rf_ScalarInteger(x) }
    }
    pub fn double(x: c_double) -> SEXP {
        unsafe { Rf_ScalarReal(x) }
    }
    pub fn logical(x: bool) -> SEXP {
        unsafe { Rf_ScalarLogical(x as c_int) }
    }

    pub fn integer_vector(xlength: R_xlen_t) -> SEXP {
        unsafe { Rf_allocVector(INTSXP, xlength) }
    }
    pub fn double_vector(xlength: R_xlen_t) -> SEXP {
        unsafe { Rf_allocVector(REALSXP, xlength) }
    }
    pub fn logical_vector(xlength: R_xlen_t) -> SEXP {
        unsafe { Rf_allocVector(LGLSXP, xlength) }
    }

    pub fn integer_matrix(nrow: c_int, ncol: c_int) -> SEXP {
        unsafe { Rf_allocMatrix(INTSXP, nrow, ncol) }
    }
    pub fn double_matrix(nrow: c_int, ncol: c_int) -> SEXP {
        unsafe { Rf_allocMatrix(REALSXP, nrow, ncol) }
    }
    pub fn logical_matrix(nrow: c_int, ncol: c_int) -> SEXP {
        unsafe { Rf_allocMatrix(LGLSXP, nrow, ncol) }
    }

    fn mk_dim_protected(dim: &[usize]) -> SEXP {
        let dim2 = R::integer_vector(dim.len() as R_xlen_t).protect();
        fn m(x: &usize) -> c_int {
            c_int::try_from(*x).unwrap()
        }
        dim2.fill_integer_from(dim, m);
        dim2
    }
    pub fn integer_array(dim: &[usize]) -> SEXP {
        let result = unsafe { Rf_allocArray(INTSXP, R::mk_dim_protected(dim)) };
        R::unprotect(1);
        result
    }
    pub fn double_array(dim: &[usize]) -> SEXP {
        let result = unsafe { Rf_allocArray(REALSXP, R::mk_dim_protected(dim)) };
        R::unprotect(1);
        result
    }
    pub fn logical_array(dim: &[usize]) -> SEXP {
        let result = unsafe { Rf_allocArray(LGLSXP, R::mk_dim_protected(dim)) };
        R::unprotect(1);
        result
    }
}

pub trait SEXPExt {
    fn protect(self) -> Self;
    fn as_integer(self) -> c_int;
    fn as_usize(self) -> usize;
    fn as_double(self) -> c_double;
    fn as_logical(self) -> c_int;
    fn as_bool(self) -> bool;
    fn as_integer_slice_mut(self) -> &'static mut [c_int];
    fn as_integer_slice(self) -> &'static [c_int];
    fn as_double_slice_mut(self) -> &'static mut [c_double];
    fn as_double_slice(self) -> &'static [c_double];
    fn as_logical_slice_mut(self) -> &'static mut [c_int];
    fn as_logical_slice(self) -> &'static [c_int];
    fn as_raw_slice_mut(self) -> &'static mut [u8];
    fn as_raw_slice(self) -> &'static [u8];
    fn fill_integer_from<T>(self, slice: &[T], mapper: fn(&T) -> c_int);
    fn fill_double_from<T>(self, slice: &[T], mapper: fn(&T) -> c_double);
    fn fill_logical_from<T>(self, slice: &[T], mapper: fn(&T) -> c_int);
    fn length(self) -> c_int;
    fn length_usize(self) -> usize;
    fn xlength(self) -> R_xlen_t;
    fn xlength_usize(self) -> usize;
    fn nrow(self) -> c_int;
    fn nrow_usize(self) -> usize;
    fn ncol(self) -> c_int;
    fn ncol_usize(self) -> usize;
    fn call0(self) -> SEXP;
    fn call1(self, x1: SEXP) -> SEXP;
    fn call2(self, x1: SEXP, x2: SEXP) -> SEXP;
    fn call3(self, x1: SEXP, x2: SEXP, x3: SEXP) -> SEXP;
    fn call4(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP) -> SEXP;
    fn call5(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP, x5: SEXP) -> SEXP;
    fn try_call0(self) -> Option<SEXP>;
    fn try_call1(self, x1: SEXP) -> Option<SEXP>;
    fn try_call2(self, x1: SEXP, x2: SEXP) -> Option<SEXP>;
    fn try_call3(self, x1: SEXP, x2: SEXP, x3: SEXP) -> Option<SEXP>;
    fn try_call4(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP) -> Option<SEXP>;
    fn try_call5(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP, x5: SEXP) -> Option<SEXP>;
}

impl SEXPExt for SEXP {
    fn protect(self) -> Self {
        unsafe { Rf_protect(self) }
    }
    fn as_integer(self) -> c_int {
        unsafe { Rf_asInteger(self) }
    }
    fn as_usize(self) -> usize {
        usize::try_from(unsafe { Rf_asInteger(self) }).unwrap()
    }
    fn as_double(self) -> c_double {
        unsafe { Rf_asReal(self) }
    }
    fn as_logical(self) -> c_int {
        unsafe { Rf_asLogical(self) }
    }
    fn as_bool(self) -> bool {
        unsafe { Rf_asLogical(self) != 0 }
    }
    fn as_integer_slice_mut(self) -> &'static mut [c_int] {
        unsafe {
            if Rf_isInteger(self) == 0 {
                panic!("Object is not an integer.")
            }
            std::slice::from_raw_parts_mut(INTEGER(self), self.xlength_usize())
        }
    }
    fn as_integer_slice(self) -> &'static [c_int] {
        unsafe {
            if Rf_isInteger(self) == 0 {
                panic!("Object is not an integer.")
            }
            std::slice::from_raw_parts(INTEGER(self), self.xlength_usize())
        }
    }
    fn as_double_slice_mut(self) -> &'static mut [c_double] {
        unsafe {
            if Rf_isReal(self) == 0 {
                panic!("Object is not a real.")
            }
            std::slice::from_raw_parts_mut(REAL(self), self.xlength_usize())
        }
    }
    fn as_double_slice(self) -> &'static [c_double] {
        unsafe {
            if Rf_isReal(self) == 0 {
                panic!("Object is not a real.")
            }
            std::slice::from_raw_parts(REAL(self), self.xlength_usize())
        }
    }
    fn as_logical_slice_mut(self) -> &'static mut [c_int] {
        unsafe {
            if Rf_isLogical(self) == 0 {
                panic!("Object is not a logical.")
            }
            std::slice::from_raw_parts_mut(LOGICAL(self), self.xlength_usize())
        }
    }
    fn as_logical_slice(self) -> &'static [c_int] {
        unsafe {
            if Rf_isLogical(self) == 0 {
                panic!("Object is not a logical.")
            }
            std::slice::from_raw_parts(LOGICAL(self), self.xlength_usize())
        }
    }
    fn as_raw_slice_mut(self) -> &'static mut [u8] {
        unsafe {
            if TYPEOF(self) == RAWSXP.try_into().unwrap() {
                panic!("Object is not a raw.")
            }
            std::slice::from_raw_parts_mut(RAW(self), self.xlength_usize())
        }
    }
    fn as_raw_slice(self) -> &'static [u8] {
        unsafe {
            if TYPEOF(self) == RAWSXP.try_into().unwrap() {
                panic!("Object is not a raw.")
            }
            std::slice::from_raw_parts(RAW(self), self.xlength_usize())
        }
    }
    fn fill_integer_from<T>(self, src: &[T], mapper: fn(&T) -> c_int) {
        let dest = self.as_integer_slice_mut();
        for (a, b) in dest.iter_mut().zip(src.iter()) {
            *a = mapper(b);
        }
    }
    fn fill_double_from<T>(self, src: &[T], mapper: fn(&T) -> c_double) {
        let dest = self.as_double_slice_mut();
        for (a, b) in dest.iter_mut().zip(src.iter()) {
            *a = mapper(b);
        }
    }
    fn fill_logical_from<T>(self, src: &[T], mapper: fn(&T) -> c_int) {
        let dest = self.as_logical_slice_mut();
        for (a, b) in dest.iter_mut().zip(src.iter()) {
            *a = mapper(b);
        }
    }
    fn length(self) -> c_int {
        unsafe { Rf_length(self) }
    }
    fn length_usize(self) -> usize {
        usize::try_from(unsafe { Rf_length(self) }).unwrap()
    }
    fn xlength(self) -> R_xlen_t {
        unsafe { Rf_xlength(self) }
    }
    fn xlength_usize(self) -> usize {
        usize::try_from(unsafe { Rf_xlength(self) }).unwrap()
    }
    fn nrow(self) -> c_int {
        unsafe { Rf_nrows(self) }
    }
    fn nrow_usize(self) -> usize {
        usize::try_from(unsafe { Rf_nrows(self) }).unwrap()
    }
    fn ncol(self) -> c_int {
        unsafe { Rf_ncols(self) }
    }
    fn ncol_usize(self) -> usize {
        usize::try_from(unsafe { Rf_ncols(self) }).unwrap()
    }
    fn call0(self) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang1(self).protect(), R_GlobalEnv);
            R::unprotect(1);
            result
        }
    }
    fn call1(self, x1: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang2(self, x1).protect(), R_GlobalEnv);
            R::unprotect(1);
            result
        }
    }
    fn call2(self, x1: SEXP, x2: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang3(self, x1, x2).protect(), R_GlobalEnv);
            R::unprotect(1);
            result
        }
    }
    fn call3(self, x1: SEXP, x2: SEXP, x3: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang4(self, x1, x2, x3).protect(), R_GlobalEnv);
            R::unprotect(1);
            result
        }
    }
    fn call4(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang5(self, x1, x2, x3, x4).protect(), R_GlobalEnv);
            R::unprotect(1);
            result
        }
    }
    fn call5(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP, x5: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang6(self, x1, x2, x3, x4, x5).protect(), R_GlobalEnv);
            R::unprotect(1);
            result
        }
    }
    fn try_call0(self) -> Option<SEXP> {
        let mut p_out_error: c_int = 0;
        let result = unsafe {
            R_tryEval(
                Rf_lang1(self).protect(),
                R_GlobalEnv,
                &mut p_out_error as *mut c_int,
            )
        };
        R::unprotect(1);
        match p_out_error {
            0 => Some(result),
            _ => None,
        }
    }
    fn try_call1(self, x1: SEXP) -> Option<SEXP> {
        let mut p_out_error: c_int = 0;
        let result = unsafe {
            R_tryEval(
                Rf_lang2(self, x1).protect(),
                R_GlobalEnv,
                &mut p_out_error as *mut c_int,
            )
        };
        R::unprotect(1);
        match p_out_error {
            0 => Some(result),
            _ => None,
        }
    }
    fn try_call2(self, x1: SEXP, x2: SEXP) -> Option<SEXP> {
        let mut p_out_error: c_int = 0;
        let result = unsafe {
            R_tryEval(
                Rf_lang3(self, x1, x2).protect(),
                R_GlobalEnv,
                &mut p_out_error as *mut c_int,
            )
        };
        R::unprotect(1);
        match p_out_error {
            0 => Some(result),
            _ => None,
        }
    }
    fn try_call3(self, x1: SEXP, x2: SEXP, x3: SEXP) -> Option<SEXP> {
        let mut p_out_error: c_int = 0;
        let result = unsafe {
            R_tryEval(
                Rf_lang4(self, x1, x2, x3).protect(),
                R_GlobalEnv,
                &mut p_out_error as *mut c_int,
            )
        };
        R::unprotect(1);
        match p_out_error {
            0 => Some(result),
            _ => None,
        }
    }
    fn try_call4(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP) -> Option<SEXP> {
        let mut p_out_error: c_int = 0;
        let result = unsafe {
            R_tryEval(
                Rf_lang5(self, x1, x2, x3, x4).protect(),
                R_GlobalEnv,
                &mut p_out_error as *mut c_int,
            )
        };
        R::unprotect(1);
        match p_out_error {
            0 => Some(result),
            _ => None,
        }
    }
    fn try_call5(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP, x5: SEXP) -> Option<SEXP> {
        let mut p_out_error: c_int = 0;
        let result = unsafe {
            R_tryEval(
                Rf_lang6(self, x1, x2, x3, x4, x5).protect(),
                R_GlobalEnv,
                &mut p_out_error as *mut c_int,
            )
        };
        R::unprotect(1);
        match p_out_error {
            0 => Some(result),
            _ => None,
        }
    }
}
