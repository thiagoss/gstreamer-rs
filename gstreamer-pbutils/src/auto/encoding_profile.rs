// This file was generated by gir (d933f9a+) from gir-files (???)
// DO NOT EDIT

use DiscovererInfo;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct EncodingProfile(Object<ffi::GstEncodingProfile, ffi::GstEncodingProfileClass>);

    match fn {
        get_type => || ffi::gst_encoding_profile_get_type(),
    }
}

impl EncodingProfile {
    pub fn find<'a, P: Into<Option<&'a str>>>(targetname: &str, profilename: &str, category: P) -> Option<EncodingProfile> {
        assert_initialized_main_thread!();
        let category = category.into();
        let category = category.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_find(targetname.to_glib_none().0, profilename.to_glib_none().0, category.0))
        }
    }

    pub fn from_discoverer(info: &DiscovererInfo) -> Option<EncodingProfile> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_from_discoverer(info.to_glib_none().0))
        }
    }
}

unsafe impl Send for EncodingProfile {}
unsafe impl Sync for EncodingProfile {}

pub trait EncodingProfileExt {
    fn copy(&self) -> Option<EncodingProfile>;

    fn get_allow_dynamic_output(&self) -> bool;

    fn get_description(&self) -> Option<String>;

    fn get_file_extension(&self) -> Option<String>;

    fn get_format(&self) -> Option<gst::Caps>;

    fn get_input_caps(&self) -> Option<gst::Caps>;

    fn get_name(&self) -> Option<String>;

    fn get_presence(&self) -> u32;

    fn get_preset(&self) -> Option<String>;

    fn get_preset_name(&self) -> Option<String>;

    fn get_restriction(&self) -> Option<gst::Caps>;

    fn get_type_nick(&self) -> Option<String>;

    fn is_enabled(&self) -> bool;

    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool);

    fn set_description(&self, description: &str);

    fn set_enabled(&self, enabled: bool);

    fn set_format(&self, format: &gst::Caps);

    fn set_name(&self, name: &str);

    fn set_presence(&self, presence: u32);

    fn set_preset<'a, P: Into<Option<&'a str>>>(&self, preset: P);

    fn set_preset_name(&self, preset_name: &str);

    fn set_restriction(&self, restriction: &gst::Caps);

    fn get_property_restriction_caps(&self) -> Option<gst::Caps>;

    fn set_property_restriction_caps(&self, restriction_caps: Option<&gst::Caps>);

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EncodingProfile> + IsA<glib::object::Object>> EncodingProfileExt for O {
    fn copy(&self) -> Option<EncodingProfile> {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_copy(self.to_glib_none().0))
        }
    }

    fn get_allow_dynamic_output(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_get_allow_dynamic_output(self.to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_description(self.to_glib_none().0))
        }
    }

    fn get_file_extension(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_file_extension(self.to_glib_none().0))
        }
    }

    fn get_format(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_format(self.to_glib_none().0))
        }
    }

    fn get_input_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_input_caps(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_name(self.to_glib_none().0))
        }
    }

    fn get_presence(&self) -> u32 {
        unsafe {
            ffi::gst_encoding_profile_get_presence(self.to_glib_none().0)
        }
    }

    fn get_preset(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_preset(self.to_glib_none().0))
        }
    }

    fn get_preset_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_preset_name(self.to_glib_none().0))
        }
    }

    fn get_restriction(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_restriction(self.to_glib_none().0))
        }
    }

    fn get_type_nick(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_type_nick(self.to_glib_none().0))
        }
    }

    fn is_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_is_enabled(self.to_glib_none().0))
        }
    }

    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_allow_dynamic_output(self.to_glib_none().0, allow_dynamic_output.to_glib());
        }
    }

    fn set_description(&self, description: &str) {
        unsafe {
            ffi::gst_encoding_profile_set_description(self.to_glib_none().0, description.to_glib_none().0);
        }
    }

    fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_format(&self, format: &gst::Caps) {
        unsafe {
            ffi::gst_encoding_profile_set_format(self.to_glib_none().0, format.to_glib_none().0);
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::gst_encoding_profile_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_presence(&self, presence: u32) {
        unsafe {
            ffi::gst_encoding_profile_set_presence(self.to_glib_none().0, presence);
        }
    }

    fn set_preset<'a, P: Into<Option<&'a str>>>(&self, preset: P) {
        let preset = preset.into();
        let preset = preset.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_preset(self.to_glib_none().0, preset.0);
        }
    }

    fn set_preset_name(&self, preset_name: &str) {
        unsafe {
            ffi::gst_encoding_profile_set_preset_name(self.to_glib_none().0, preset_name.to_glib_none().0);
        }
    }

    fn set_restriction(&self, restriction: &gst::Caps) {
        unsafe {
            ffi::gst_encoding_profile_set_restriction(self.to_glib_none().0, restriction.to_glib_full());
        }
    }

    fn get_property_restriction_caps(&self) -> Option<gst::Caps> {
        let mut value = Value::from(None::<&gst::Caps>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "restriction-caps".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_restriction_caps(&self, restriction_caps: Option<&gst::Caps>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "restriction-caps".to_glib_none().0, Value::from(restriction_caps).to_glib_none().0);
        }
    }

    fn connect_property_restriction_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::restriction-caps",
                transmute(notify_restriction_caps_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_restriction_caps_trampoline<P>(this: *mut ffi::GstEncodingProfile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<EncodingProfile> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&EncodingProfile::from_glib_borrow(this).downcast_unchecked())
}
