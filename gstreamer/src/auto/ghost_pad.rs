// This file was generated by gir (3294959) from gir-files (???)
// DO NOT EDIT

use Object;
use Pad;
use PadDirection;
use PadMode;
use PadTemplate;
use ProxyPad;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GhostPad(Object<ffi::GstGhostPad>): ProxyPad, Pad, Object;

    match fn {
        get_type => || ffi::gst_ghost_pad_get_type(),
    }
}

impl GhostPad {
    pub fn new_no_target<'a, P: Into<Option<&'a str>>>(name: P, dir: PadDirection) -> GhostPad {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Pad::from_glib_none(ffi::gst_ghost_pad_new_no_target(name.0, dir.to_glib())).downcast_unchecked()
        }
    }

    pub fn new_no_target_from_template<'a, P: Into<Option<&'a str>>>(name: P, templ: &PadTemplate) -> GhostPad {
        skip_assert_initialized!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Pad::from_glib_none(ffi::gst_ghost_pad_new_no_target_from_template(name.0, templ.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn activate_mode_default<'a, P: IsA<Pad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(pad: &P, parent: R, mode: PadMode, active: bool) -> bool {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::gst_ghost_pad_activate_mode_default(pad.to_glib_none().0, parent.0, mode.to_glib(), active.to_glib()))
        }
    }

    pub fn internal_activate_mode_default<'a, P: IsA<Pad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(pad: &P, parent: R, mode: PadMode, active: bool) -> bool {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::gst_ghost_pad_internal_activate_mode_default(pad.to_glib_none().0, parent.0, mode.to_glib(), active.to_glib()))
        }
    }
}

unsafe impl Send for GhostPad {}
unsafe impl Sync for GhostPad {}

pub trait GhostPadExt {
    fn get_target(&self) -> Option<Pad>;

    fn set_target<'a, P: IsA<Pad> + 'a, Q: Into<Option<&'a P>>>(&self, newtarget: Q) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<GhostPad>> GhostPadExt for O {
    fn get_target(&self) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_ghost_pad_get_target(self.to_glib_none().0))
        }
    }

    fn set_target<'a, P: IsA<Pad> + 'a, Q: Into<Option<&'a P>>>(&self, newtarget: Q) -> Result<(), glib::error::BoolError> {
        let newtarget = newtarget.into();
        let newtarget = newtarget.to_glib_none();
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_ghost_pad_set_target(self.to_glib_none().0, newtarget.0), "Failed to set target")
        }
    }
}
