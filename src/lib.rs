#![allow(dead_code)]

// Help: https://docs.rs/libR-sys, https://github.com/hadley/r-internals
use libR_sys::*;

pub use libR_sys;
pub use libR_sys::SEXP;
use std::os::raw::{c_char, c_int};

pub struct SEXPMethods;

impl SEXPMethods {
    // Use to seed a RNG from R.
    pub fn random_bytes_from_r<const LENGTH: usize>() -> [u8; LENGTH] {
        unsafe {
            let result = Rf_install(b"sample.int\0".as_ptr() as *const c_char)
                .protect()
                .call2(
                    SEXPMethods::integer((u8::MAX as c_int) + 1).protect(),
                    SEXPMethods::integer(LENGTH as c_int).protect(),
                )
                .protect();
            let slice = result.as_integer_slice();
            let mut bytes: [u8; LENGTH] = [0; LENGTH];
            bytes
                .iter_mut()
                .zip(slice)
                .for_each(|(b, s)| *b = (*s - 1) as u8);
            SEXPMethods::unprotect(4);
            bytes
        }
    }
    pub unsafe fn print_str(x: &str) {
        Rprintf(
            b"%.*s\0".as_ptr() as *const c_char,
            x.len(),
            x.as_ptr() as *const c_char,
        );
    }
    pub fn unprotect(i: i32) {
        unsafe { Rf_unprotect(i) }
    }
    pub fn integer(x: i32) -> SEXP {
        unsafe { Rf_ScalarInteger(x) }
    }
    pub fn double(x: f64) -> SEXP {
        unsafe { Rf_ScalarReal(x) }
    }
    pub fn logical(x: bool) -> SEXP {
        unsafe { Rf_ScalarLogical(x as c_int) }
    }
    pub fn integer_vector(xlength: isize) -> SEXP {
        unsafe { Rf_allocVector(INTSXP, xlength) }
    }
    pub fn double_vector(xlength: isize) -> SEXP {
        unsafe { Rf_allocVector(REALSXP, xlength) }
    }
    pub fn logical_vector(xlength: isize) -> SEXP {
        unsafe { Rf_allocVector(LGLSXP, xlength) }
    }
    pub fn integer_matrix(nrow: i32, ncol: i32) -> SEXP {
        unsafe { Rf_allocMatrix(INTSXP, nrow, ncol) }
    }
    pub fn double_matrix(nrow: i32, ncol: i32) -> SEXP {
        unsafe { Rf_allocMatrix(REALSXP, nrow, ncol) }
    }
    pub fn logical_matrix(nrow: i32, ncol: i32) -> SEXP {
        unsafe { Rf_allocMatrix(LGLSXP, nrow, ncol) }
    }
}

pub trait SEXPExt {
    fn protect(self) -> Self;
    fn as_integer(self) -> i32;
    fn as_double(self) -> f64;
    fn as_logical(self) -> i32;
    fn as_bool(self) -> bool;
    fn as_integer_slice_mut(self) -> &'static mut [i32];
    fn as_integer_slice(self) -> &'static [i32];
    fn as_double_slice_mut(self) -> &'static mut [f64];
    fn as_double_slice(self) -> &'static [f64];
    fn as_logical_slice_mut(self) -> &'static mut [i32];
    fn as_logical_slice(self) -> &'static [i32];
    fn as_raw_slice_mut(self) -> &'static mut [u8];
    fn as_raw_slice(self) -> &'static [u8];
    fn length(self) -> i32;
    fn xlength(self) -> isize;
    fn nrow(self) -> i32;
    fn ncol(self) -> i32;
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
    fn as_integer(self) -> i32 {
        unsafe { Rf_asInteger(self) }
    }
    fn as_double(self) -> f64 {
        unsafe { Rf_asReal(self) }
    }
    fn as_logical(self) -> i32 {
        unsafe { Rf_asLogical(self) }
    }
    fn as_bool(self) -> bool {
        unsafe { Rf_asLogical(self) != 0 }
    }
    fn as_integer_slice_mut(self) -> &'static mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(INTEGER(self), self.xlength() as usize) }
    }
    fn as_integer_slice(self) -> &'static [i32] {
        unsafe { std::slice::from_raw_parts(INTEGER(self), self.xlength() as usize) }
    }
    fn as_double_slice_mut(self) -> &'static mut [f64] {
        unsafe { std::slice::from_raw_parts_mut(REAL(self), self.xlength() as usize) }
    }
    fn as_double_slice(self) -> &'static [f64] {
        unsafe { std::slice::from_raw_parts(REAL(self), self.xlength() as usize) }
    }
    fn as_logical_slice_mut(self) -> &'static mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(LOGICAL(self), self.xlength() as usize) }
    }
    fn as_logical_slice(self) -> &'static [i32] {
        unsafe { std::slice::from_raw_parts(LOGICAL(self), self.xlength() as usize) }
    }
    fn as_raw_slice_mut(self) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(RAW(self), self.xlength() as usize) }
    }
    fn as_raw_slice(self) -> &'static [u8] {
        unsafe { std::slice::from_raw_parts(RAW(self), self.xlength() as usize) }
    }
    fn length(self) -> i32 {
        unsafe { Rf_length(self) }
    }
    fn xlength(self) -> isize {
        unsafe { Rf_xlength(self) }
    }
    fn nrow(self) -> i32 {
        unsafe { Rf_nrows(self) }
    }
    fn ncol(self) -> i32 {
        unsafe { Rf_ncols(self) }
    }
    fn call0(self) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang1(self).protect(), R_GlobalEnv);
            SEXPMethods::unprotect(1);
            result
        }
    }
    fn call1(self, x1: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang2(self, x1).protect(), R_GlobalEnv);
            SEXPMethods::unprotect(1);
            result
        }
    }
    fn call2(self, x1: SEXP, x2: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang3(self, x1, x2).protect(), R_GlobalEnv);
            SEXPMethods::unprotect(1);
            result
        }
    }
    fn call3(self, x1: SEXP, x2: SEXP, x3: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang4(self, x1, x2, x3).protect(), R_GlobalEnv);
            SEXPMethods::unprotect(1);
            result
        }
    }
    fn call4(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang5(self, x1, x2, x3, x4).protect(), R_GlobalEnv);
            SEXPMethods::unprotect(1);
            result
        }
    }
    fn call5(self, x1: SEXP, x2: SEXP, x3: SEXP, x4: SEXP, x5: SEXP) -> SEXP {
        unsafe {
            let result = Rf_eval(Rf_lang6(self, x1, x2, x3, x4, x5).protect(), R_GlobalEnv);
            SEXPMethods::unprotect(1);
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
        SEXPMethods::unprotect(1);
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
        SEXPMethods::unprotect(1);
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
        SEXPMethods::unprotect(1);
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
        SEXPMethods::unprotect(1);
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
        SEXPMethods::unprotect(1);
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
        SEXPMethods::unprotect(1);
        match p_out_error {
            0 => Some(result),
            _ => None,
        }
    }
}
