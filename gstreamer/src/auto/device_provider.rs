// This file was generated by gir (3294959) from gir-files (???)
// DO NOT EDIT

use Bus;
use Device;
use DeviceProviderFactory;
use Object;
use Plugin;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DeviceProvider(Object<ffi::GstDeviceProvider>): Object;

    match fn {
        get_type => || ffi::gst_device_provider_get_type(),
    }
}

impl DeviceProvider {
    pub fn register<'a, P: Into<Option<&'a Plugin>>>(plugin: P, name: &str, rank: u32, type_: glib::types::Type) -> bool {
        assert_initialized_main_thread!();
        let plugin = plugin.into();
        let plugin = plugin.to_glib_none();
        unsafe {
            from_glib(ffi::gst_device_provider_register(plugin.0, name.to_glib_none().0, rank, type_.to_glib()))
        }
    }
}

unsafe impl Send for DeviceProvider {}
unsafe impl Sync for DeviceProvider {}

pub trait DeviceProviderExt {
    fn can_monitor(&self) -> bool;

    fn device_add(&self, device: &Device);

    fn device_remove(&self, device: &Device);

    fn get_bus(&self) -> Option<Bus>;

    fn get_devices(&self) -> Vec<Device>;

    fn get_factory(&self) -> Option<DeviceProviderFactory>;

    fn get_hidden_providers(&self) -> Vec<String>;

    fn hide_provider(&self, name: &str);

    fn start(&self) -> bool;

    fn stop(&self);

    fn unhide_provider(&self, name: &str);

    fn connect_provider_hidden<F: Fn(&Self, &str) + Send + Sync + 'static>(&self, f: F) -> u64;

    fn connect_provider_unhidden<F: Fn(&Self, &str) + Send + Sync + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DeviceProvider> + IsA<glib::object::Object>> DeviceProviderExt for O {
    fn can_monitor(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_can_monitor(self.to_glib_none().0))
        }
    }

    fn device_add(&self, device: &Device) {
        unsafe {
            ffi::gst_device_provider_device_add(self.to_glib_none().0, device.to_glib_none().0);
        }
    }

    fn device_remove(&self, device: &Device) {
        unsafe {
            ffi::gst_device_provider_device_remove(self.to_glib_none().0, device.to_glib_none().0);
        }
    }

    fn get_bus(&self) -> Option<Bus> {
        unsafe {
            from_glib_full(ffi::gst_device_provider_get_bus(self.to_glib_none().0))
        }
    }

    fn get_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_get_devices(self.to_glib_none().0))
        }
    }

    fn get_factory(&self) -> Option<DeviceProviderFactory> {
        unsafe {
            from_glib_none(ffi::gst_device_provider_get_factory(self.to_glib_none().0))
        }
    }

    fn get_hidden_providers(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_get_hidden_providers(self.to_glib_none().0))
        }
    }

    fn hide_provider(&self, name: &str) {
        unsafe {
            ffi::gst_device_provider_hide_provider(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn start(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_start(self.to_glib_none().0))
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gst_device_provider_stop(self.to_glib_none().0);
        }
    }

    fn unhide_provider(&self, name: &str) {
        unsafe {
            ffi::gst_device_provider_unhide_provider(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn connect_provider_hidden<F: Fn(&Self, &str) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "provider-hidden",
                transmute(provider_hidden_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_provider_unhidden<F: Fn(&Self, &str) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "provider-unhidden",
                transmute(provider_unhidden_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn provider_hidden_trampoline<P>(this: *mut ffi::GstDeviceProvider, object: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<DeviceProvider> {
    callback_guard!();
    let f: &&(Fn(&P, &str) + Send + Sync + 'static) = transmute(f);
    f(&DeviceProvider::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(object))
}

unsafe extern "C" fn provider_unhidden_trampoline<P>(this: *mut ffi::GstDeviceProvider, object: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<DeviceProvider> {
    callback_guard!();
    let f: &&(Fn(&P, &str) + Send + Sync + 'static) = transmute(f);
    f(&DeviceProvider::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(object))
}
