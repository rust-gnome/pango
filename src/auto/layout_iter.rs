// This file was generated by gir (add4ad6) from gir-files (0bcaef9)
// DO NOT EDIT

use Layout;
use LayoutLine;
use Rectangle;
use ffi;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct LayoutIter(Boxed<ffi::PangoLayoutIter>);

    match fn {
        copy => |ptr| ffi::pango_layout_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_layout_iter_free(ptr),
    }
}

impl LayoutIter {
    pub fn at_last_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_at_last_line(self.to_glib_none_mut().0))
        }
    }

    pub fn get_baseline(&mut self) -> i32 {
        unsafe {
            ffi::pango_layout_iter_get_baseline(self.to_glib_none_mut().0)
        }
    }

    pub fn get_char_extents(&mut self) -> Rectangle {
        unsafe {
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_char_extents(self.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            logical_rect
        }
    }

    pub fn get_cluster_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_cluster_extents(self.to_glib_none_mut().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    pub fn get_index(&mut self) -> i32 {
        unsafe {
            ffi::pango_layout_iter_get_index(self.to_glib_none_mut().0)
        }
    }

    pub fn get_layout(&mut self) -> Option<Layout> {
        unsafe {
            from_glib_none(ffi::pango_layout_iter_get_layout(self.to_glib_none_mut().0))
        }
    }

    pub fn get_layout_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_layout_extents(self.to_glib_none_mut().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    pub fn get_line(&mut self) -> Option<LayoutLine> {
        unsafe {
            from_glib_full(ffi::pango_layout_iter_get_line(self.to_glib_none_mut().0))
        }
    }

    pub fn get_line_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_line_extents(self.to_glib_none_mut().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    pub fn get_line_readonly(&mut self) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_iter_get_line_readonly(self.to_glib_none_mut().0))
        }
    }

    pub fn get_line_yrange(&mut self) -> (i32, i32) {
        unsafe {
            let mut y0_ = mem::uninitialized();
            let mut y1_ = mem::uninitialized();
            ffi::pango_layout_iter_get_line_yrange(self.to_glib_none_mut().0, &mut y0_, &mut y1_);
            (y0_, y1_)
        }
    }

    //pub fn get_run(&mut self) -> /*Ignored*/Option<LayoutRun> {
    //    unsafe { TODO: call ffi::pango_layout_iter_get_run() }
    //}

    pub fn get_run_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_run_extents(self.to_glib_none_mut().0, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    //pub fn get_run_readonly(&mut self) -> /*Ignored*/Option<LayoutRun> {
    //    unsafe { TODO: call ffi::pango_layout_iter_get_run_readonly() }
    //}

    pub fn next_char(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_next_char(self.to_glib_none_mut().0))
        }
    }

    pub fn next_cluster(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_next_cluster(self.to_glib_none_mut().0))
        }
    }

    pub fn next_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_next_line(self.to_glib_none_mut().0))
        }
    }

    pub fn next_run(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_next_run(self.to_glib_none_mut().0))
        }
    }
}
