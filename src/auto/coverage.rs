// This file was generated by gir (add4ad6) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct Coverage(Shared<ffi::PangoCoverage>);

    match fn {
        ref => |ptr| ffi::pango_coverage_ref(ptr),
        unref => |ptr| ffi::pango_coverage_unref(ptr),
    }
}

impl Coverage {
    pub fn copy(&self) -> Option<Coverage> {
        unsafe {
            from_glib_full(ffi::pango_coverage_copy(self.to_glib_none().0))
        }
    }

    //pub fn get(&self, index_: i32) -> /*Ignored*/CoverageLevel {
    //    unsafe { TODO: call ffi::pango_coverage_get() }
    //}

    pub fn max(&self, other: &Coverage) {
        unsafe {
            ffi::pango_coverage_max(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    //pub fn set(&self, index_: i32, level: /*Ignored*/CoverageLevel) {
    //    unsafe { TODO: call ffi::pango_coverage_set() }
    //}

    //pub fn to_bytes(&self, bytes: /*Unimplemented*/CArray TypeId { ns_id: 0, id: 3 }) -> i32 {
    //    unsafe { TODO: call ffi::pango_coverage_to_bytes() }
    //}

    //pub fn from_bytes(bytes: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }, n_bytes: i32) -> Option<Coverage> {
    //    unsafe { TODO: call ffi::pango_coverage_from_bytes() }
    //}

    pub fn new() -> Option<Coverage> {
        unsafe {
            from_glib_none(ffi::pango_coverage_new())
        }
    }
}
