// This file was generated by gir (https://github.com/gtk-rs/gir @ 47eb915)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use DiscovererStreamInfo;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DiscovererVideoInfo(Object<ffi::GstDiscovererVideoInfo>): DiscovererStreamInfo;

    match fn {
        get_type => || ffi::gst_discoverer_video_info_get_type(),
    }
}

impl DiscovererVideoInfo {
    pub fn get_bitrate(&self) -> u32 {
        unsafe {
            ffi::gst_discoverer_video_info_get_bitrate(self.to_glib_none().0)
        }
    }

    pub fn get_depth(&self) -> u32 {
        unsafe {
            ffi::gst_discoverer_video_info_get_depth(self.to_glib_none().0)
        }
    }

    pub fn get_height(&self) -> u32 {
        unsafe {
            ffi::gst_discoverer_video_info_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_max_bitrate(&self) -> u32 {
        unsafe {
            ffi::gst_discoverer_video_info_get_max_bitrate(self.to_glib_none().0)
        }
    }

    pub fn get_width(&self) -> u32 {
        unsafe {
            ffi::gst_discoverer_video_info_get_width(self.to_glib_none().0)
        }
    }

    pub fn is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_discoverer_video_info_is_image(self.to_glib_none().0))
        }
    }

    pub fn is_interlaced(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_discoverer_video_info_is_interlaced(self.to_glib_none().0))
        }
    }
}

unsafe impl Send for DiscovererVideoInfo {}
unsafe impl Sync for DiscovererVideoInfo {}
