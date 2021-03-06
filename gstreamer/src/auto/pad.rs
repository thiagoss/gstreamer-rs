// This file was generated by gir (d50d839) from gir-files (???)
// DO NOT EDIT

use Caps;
use Element;
use Event;
use EventType;
use FlowReturn;
use Object;
use PadDirection;
use PadLinkCheck;
use PadLinkReturn;
use PadMode;
use PadTemplate;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Stream;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use TaskState;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
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
    pub struct Pad(Object<ffi::GstPad, ffi::GstPadClass>): Object;

    match fn {
        get_type => || ffi::gst_pad_get_type(),
    }
}

impl Pad {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P, direction: PadDirection) -> Pad {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gst_pad_new(name.0, direction.to_glib()))
        }
    }

    pub fn new_from_template<'a, P: Into<Option<&'a str>>>(templ: &PadTemplate, name: P) -> Pad {
        skip_assert_initialized!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gst_pad_new_from_template(templ.to_glib_none().0, name.0))
        }
    }
}

unsafe impl Send for Pad {}
unsafe impl Sync for Pad {}

pub trait PadExt {
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError>;

    //fn add_probe<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mask: PadProbeType, callback: /*Unknown conversion*//*Unimplemented*/PadProbeCallback, user_data: P, destroy_data: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> libc::c_ulong;

    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool;

    fn check_reconfigure(&self) -> bool;

    fn create_stream_id<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q) -> Option<String>;

    //fn create_stream_id_printf<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<String>;

    //fn create_stream_id_printf_valist<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<String>;

    //fn forward<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, forward: /*Unknown conversion*//*Unimplemented*/PadForwardFunction, user_data: P) -> bool;

    fn get_allowed_caps(&self) -> Option<Caps>;

    fn get_current_caps(&self) -> Option<Caps>;

    fn get_direction(&self) -> PadDirection;

    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn get_last_flow_return(&self) -> FlowReturn;

    fn get_offset(&self) -> i64;

    fn get_pad_template(&self) -> Option<PadTemplate>;

    fn get_pad_template_caps(&self) -> Option<Caps>;

    fn get_parent_element(&self) -> Option<Element>;

    fn get_peer(&self) -> Option<Pad>;

    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_stream(&self) -> Option<Stream>;

    fn get_stream_id(&self) -> Option<String>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_task_state(&self) -> TaskState;

    fn has_current_caps(&self) -> bool;

    fn is_active(&self) -> bool;

    fn is_blocked(&self) -> bool;

    fn is_blocking(&self) -> bool;

    fn is_linked(&self) -> bool;

    //fn iterate_internal_links(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_internal_links_default<'a, P: IsA<Object> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) -> /*Ignored*/Option<Iterator>;

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> PadLinkReturn;

    fn link_full<P: IsA<Pad>>(&self, sinkpad: &P, flags: PadLinkCheck) -> PadLinkReturn;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting_full<P: IsA<Pad>>(&self, sink: &P, flags: PadLinkCheck) -> bool;

    fn mark_reconfigure(&self);

    fn needs_reconfigure(&self) -> bool;

    fn pause_task(&self) -> Result<(), glib::error::BoolError>;

    fn peer_query_accept_caps(&self, caps: &Caps) -> bool;

    fn peer_query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps>;

    fn query_accept_caps(&self, caps: &Caps) -> bool;

    fn query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps>;

    //fn set_activate_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activate: /*Unknown conversion*//*Unimplemented*/PadActivateFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_activatemode_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activatemode: /*Unknown conversion*//*Unimplemented*/PadActivateModeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    //fn set_chain_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chain: /*Unknown conversion*//*Unimplemented*/PadChainFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_chain_list_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chainlist: /*Unknown conversion*//*Unimplemented*/PadChainListFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_element_private<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, priv_: P);

    //fn set_event_full_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFullFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_event_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_getrange_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, get: /*Unknown conversion*//*Unimplemented*/PadGetRangeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_iterate_internal_links_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, iterintlink: /*Unknown conversion*//*Unimplemented*/PadIterIntLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_link_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, link: /*Unknown conversion*//*Unimplemented*/PadLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_offset(&self, offset: i64);

    //fn set_query_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, query: /*Unknown conversion*//*Unimplemented*/PadQueryFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_unlink_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, unlink: /*Unknown conversion*//*Unimplemented*/PadUnlinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn start_task<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TaskFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> bool;

    //fn sticky_events_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, foreach_func: /*Unknown conversion*//*Unimplemented*/PadStickyEventsForeachFunction, user_data: P);

    fn stop_task(&self) -> Result<(), glib::error::BoolError>;

    fn store_sticky_event(&self, event: &Event) -> FlowReturn;

    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError>;

    fn use_fixed_caps(&self);

    fn get_property_caps(&self) -> Option<Caps>;

    fn get_property_template(&self) -> Option<PadTemplate>;

    fn set_property_template(&self, template: Option<&PadTemplate>);

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_template_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Pad> + IsA<glib::object::Object>> PadExt for O {
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_activate_mode(self.to_glib_none().0, mode.to_glib(), active.to_glib()), "Failed to activate mode pad")
        }
    }

    //fn add_probe<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mask: PadProbeType, callback: /*Unknown conversion*//*Unimplemented*/PadProbeCallback, user_data: P, destroy_data: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> libc::c_ulong {
    //    unsafe { TODO: call ffi::gst_pad_add_probe() }
    //}

    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_can_link(self.to_glib_none().0, sinkpad.to_glib_none().0))
        }
    }

    fn check_reconfigure(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_check_reconfigure(self.to_glib_none().0))
        }
    }

    fn create_stream_id<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q) -> Option<String> {
        let stream_id = stream_id.into();
        let stream_id = stream_id.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_create_stream_id(self.to_glib_none().0, parent.to_glib_none().0, stream_id.0))
        }
    }

    //fn create_stream_id_printf<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<String> {
    //    unsafe { TODO: call ffi::gst_pad_create_stream_id_printf() }
    //}

    //fn create_stream_id_printf_valist<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<String> {
    //    unsafe { TODO: call ffi::gst_pad_create_stream_id_printf_valist() }
    //}

    //fn forward<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, forward: /*Unknown conversion*//*Unimplemented*/PadForwardFunction, user_data: P) -> bool {
    //    unsafe { TODO: call ffi::gst_pad_forward() }
    //}

    fn get_allowed_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_allowed_caps(self.to_glib_none().0))
        }
    }

    fn get_current_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_current_caps(self.to_glib_none().0))
        }
    }

    fn get_direction(&self) -> PadDirection {
        unsafe {
            from_glib(ffi::gst_pad_get_direction(self.to_glib_none().0))
        }
    }

    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gst_pad_get_element_private() }
    //}

    fn get_last_flow_return(&self) -> FlowReturn {
        unsafe {
            from_glib(ffi::gst_pad_get_last_flow_return(self.to_glib_none().0))
        }
    }

    fn get_offset(&self) -> i64 {
        unsafe {
            ffi::gst_pad_get_offset(self.to_glib_none().0)
        }
    }

    fn get_pad_template(&self) -> Option<PadTemplate> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_pad_template(self.to_glib_none().0))
        }
    }

    fn get_pad_template_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_pad_template_caps(self.to_glib_none().0))
        }
    }

    fn get_parent_element(&self) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_parent_element(self.to_glib_none().0))
        }
    }

    fn get_peer(&self) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_peer(self.to_glib_none().0))
        }
    }

    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_sticky_event(self.to_glib_none().0, event_type.to_glib(), idx))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_stream(&self) -> Option<Stream> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_stream(self.to_glib_none().0))
        }
    }

    fn get_stream_id(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_stream_id(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_task_state(&self) -> TaskState {
        unsafe {
            from_glib(ffi::gst_pad_get_task_state(self.to_glib_none().0))
        }
    }

    fn has_current_caps(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_has_current_caps(self.to_glib_none().0))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_active(self.to_glib_none().0))
        }
    }

    fn is_blocked(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_blocked(self.to_glib_none().0))
        }
    }

    fn is_blocking(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_blocking(self.to_glib_none().0))
        }
    }

    fn is_linked(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_linked(self.to_glib_none().0))
        }
    }

    //fn iterate_internal_links(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_pad_iterate_internal_links() }
    //}

    //fn iterate_internal_links_default<'a, P: IsA<Object> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi::gst_pad_iterate_internal_links_default() }
    //}

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> PadLinkReturn {
        unsafe {
            from_glib(ffi::gst_pad_link(self.to_glib_none().0, sinkpad.to_glib_none().0))
        }
    }

    fn link_full<P: IsA<Pad>>(&self, sinkpad: &P, flags: PadLinkCheck) -> PadLinkReturn {
        unsafe {
            from_glib(ffi::gst_pad_link_full(self.to_glib_none().0, sinkpad.to_glib_none().0, flags.to_glib()))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_link_maybe_ghosting(self.to_glib_none().0, sink.to_glib_none().0), "Failed to link pad, possibly ghosting")
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting_full<P: IsA<Pad>>(&self, sink: &P, flags: PadLinkCheck) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_link_maybe_ghosting_full(self.to_glib_none().0, sink.to_glib_none().0, flags.to_glib()))
        }
    }

    fn mark_reconfigure(&self) {
        unsafe {
            ffi::gst_pad_mark_reconfigure(self.to_glib_none().0);
        }
    }

    fn needs_reconfigure(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_needs_reconfigure(self.to_glib_none().0))
        }
    }

    fn pause_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_pause_task(self.to_glib_none().0), "Failed to pause pad task")
        }
    }

    fn peer_query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_peer_query_accept_caps(self.to_glib_none().0, caps.to_glib_none().0))
        }
    }

    fn peer_query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_peer_query_caps(self.to_glib_none().0, filter.0))
        }
    }

    fn query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_query_accept_caps(self.to_glib_none().0, caps.to_glib_none().0))
        }
    }

    fn query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_query_caps(self.to_glib_none().0, filter.0))
        }
    }

    //fn set_activate_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activate: /*Unknown conversion*//*Unimplemented*/PadActivateFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_activate_function_full() }
    //}

    //fn set_activatemode_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activatemode: /*Unknown conversion*//*Unimplemented*/PadActivateModeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_activatemode_function_full() }
    //}

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_set_active(self.to_glib_none().0, active.to_glib()), "Failed to activate pad")
        }
    }

    //fn set_chain_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chain: /*Unknown conversion*//*Unimplemented*/PadChainFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_chain_function_full() }
    //}

    //fn set_chain_list_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chainlist: /*Unknown conversion*//*Unimplemented*/PadChainListFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_chain_list_function_full() }
    //}

    //fn set_element_private<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, priv_: P) {
    //    unsafe { TODO: call ffi::gst_pad_set_element_private() }
    //}

    //fn set_event_full_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFullFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_event_full_function_full() }
    //}

    //fn set_event_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_event_function_full() }
    //}

    //fn set_getrange_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, get: /*Unknown conversion*//*Unimplemented*/PadGetRangeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_getrange_function_full() }
    //}

    //fn set_iterate_internal_links_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, iterintlink: /*Unknown conversion*//*Unimplemented*/PadIterIntLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_iterate_internal_links_function_full() }
    //}

    //fn set_link_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, link: /*Unknown conversion*//*Unimplemented*/PadLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_link_function_full() }
    //}

    fn set_offset(&self, offset: i64) {
        unsafe {
            ffi::gst_pad_set_offset(self.to_glib_none().0, offset);
        }
    }

    //fn set_query_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, query: /*Unknown conversion*//*Unimplemented*/PadQueryFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_query_function_full() }
    //}

    //fn set_unlink_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, unlink: /*Unknown conversion*//*Unimplemented*/PadUnlinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_unlink_function_full() }
    //}

    //fn start_task<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TaskFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> bool {
    //    unsafe { TODO: call ffi::gst_pad_start_task() }
    //}

    //fn sticky_events_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, foreach_func: /*Unknown conversion*//*Unimplemented*/PadStickyEventsForeachFunction, user_data: P) {
    //    unsafe { TODO: call ffi::gst_pad_sticky_events_foreach() }
    //}

    fn stop_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_stop_task(self.to_glib_none().0), "Failed to stop pad task")
        }
    }

    fn store_sticky_event(&self, event: &Event) -> FlowReturn {
        unsafe {
            from_glib(ffi::gst_pad_store_sticky_event(self.to_glib_none().0, event.to_glib_none().0))
        }
    }

    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_unlink(self.to_glib_none().0, sinkpad.to_glib_none().0), "Failed to unlink pad")
        }
    }

    fn use_fixed_caps(&self) {
        unsafe {
            ffi::gst_pad_use_fixed_caps(self.to_glib_none().0);
        }
    }

    fn get_property_caps(&self) -> Option<Caps> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <Caps as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "caps".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_template(&self) -> Option<PadTemplate> {
        unsafe {
            let mut value = Value::uninitialized();
            gobject_ffi::g_value_init(value.to_glib_none_mut().0, <PadTemplate as StaticType>::static_type().to_glib());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "template".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_template(&self, template: Option<&PadTemplate>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "template".to_glib_none().0, Value::from(template).to_glib_none().0);
        }
    }

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Pad) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "linked",
                transmute(linked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Pad) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unlinked",
                transmute(unlinked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::caps",
                transmute(notify_caps_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::direction",
                transmute(notify_direction_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::offset",
                transmute(notify_offset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_template_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::template",
                transmute(notify_template_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn linked_trampoline<P>(this: *mut ffi::GstPad, peer: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P, &Pad) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(peer))
}

unsafe extern "C" fn unlinked_trampoline<P>(this: *mut ffi::GstPad, peer: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P, &Pad) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(peer))
}

unsafe extern "C" fn notify_caps_trampoline<P>(this: *mut ffi::GstPad, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_direction_trampoline<P>(this: *mut ffi::GstPad, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_offset_trampoline<P>(this: *mut ffi::GstPad, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_template_trampoline<P>(this: *mut ffi::GstPad, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_borrow(this).downcast_unchecked())
}
