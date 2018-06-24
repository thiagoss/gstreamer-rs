// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use ChildProxy;
use Clock;
use ClockTime;
use Element;
use Object;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Pipeline(Object<ffi::GstPipeline, ffi::GstPipelineClass>): Bin, Element, Object, ChildProxy;

    match fn {
        get_type => || ffi::gst_pipeline_get_type(),
    }
}

impl Pipeline {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P) -> Pipeline {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Element::from_glib_none(ffi::gst_pipeline_new(name.0)).downcast_unchecked()
        }
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

pub trait PipelineExt {
    fn auto_clock(&self);

    fn get_auto_flush_bus(&self) -> bool;

    fn get_delay(&self) -> ClockTime;

    fn get_latency(&self) -> ClockTime;

    fn get_pipeline_clock(&self) -> Option<Clock>;

    fn set_auto_flush_bus(&self, auto_flush: bool);

    fn set_delay(&self, delay: ClockTime);

    fn set_latency(&self, latency: ClockTime);

    fn use_clock<'a, P: IsA<Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q);

    fn connect_property_auto_flush_bus_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Pipeline> + IsA<glib::object::Object>> PipelineExt for O {
    fn auto_clock(&self) {
        unsafe {
            ffi::gst_pipeline_auto_clock(self.to_glib_none().0);
        }
    }

    fn get_auto_flush_bus(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pipeline_get_auto_flush_bus(self.to_glib_none().0))
        }
    }

    fn get_delay(&self) -> ClockTime {
        unsafe {
            from_glib(ffi::gst_pipeline_get_delay(self.to_glib_none().0))
        }
    }

    fn get_latency(&self) -> ClockTime {
        unsafe {
            from_glib(ffi::gst_pipeline_get_latency(self.to_glib_none().0))
        }
    }

    fn get_pipeline_clock(&self) -> Option<Clock> {
        unsafe {
            from_glib_full(ffi::gst_pipeline_get_pipeline_clock(self.to_glib_none().0))
        }
    }

    fn set_auto_flush_bus(&self, auto_flush: bool) {
        unsafe {
            ffi::gst_pipeline_set_auto_flush_bus(self.to_glib_none().0, auto_flush.to_glib());
        }
    }

    fn set_delay(&self, delay: ClockTime) {
        unsafe {
            ffi::gst_pipeline_set_delay(self.to_glib_none().0, delay.to_glib());
        }
    }

    fn set_latency(&self, latency: ClockTime) {
        unsafe {
            ffi::gst_pipeline_set_latency(self.to_glib_none().0, latency.to_glib());
        }
    }

    fn use_clock<'a, P: IsA<Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q) {
        let clock = clock.into();
        let clock = clock.to_glib_none();
        unsafe {
            ffi::gst_pipeline_use_clock(self.to_glib_none().0, clock.0);
        }
    }

    fn connect_property_auto_flush_bus_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::auto-flush-bus",
                transmute(notify_auto_flush_bus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::delay",
                transmute(notify_delay_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::latency",
                transmute(notify_latency_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_auto_flush_bus_trampoline<P>(this: *mut ffi::GstPipeline, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pipeline> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Pipeline::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_delay_trampoline<P>(this: *mut ffi::GstPipeline, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pipeline> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Pipeline::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_latency_trampoline<P>(this: *mut ffi::GstPipeline, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pipeline> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Pipeline::from_glib_borrow(this).downcast_unchecked())
}
