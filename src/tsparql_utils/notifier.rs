use std::mem;

use gtk::glib::{self, closure_local, prelude::*, Value};

use tsparql::{Notifier, NotifierEvent};

pub trait NotifierUtils {
    fn connect_events<F: Fn(&Self, Option<&str>, Option<&str>, Vec<NotifierEvent>) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId;
}

impl NotifierUtils for Notifier {
    /// Connect to the signal emitted when events are received.
    fn connect_events<F: Fn(&Self, Option<&str>, Option<&str>, Vec<NotifierEvent>) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_closure(
            "events",
            true,
            closure_local!(move |obj: Self,
                                 service: Option<&str>,
                                 graph: Option<&str>,
                                 events: Value| {
                let events: Vec<NotifierEvent> = {
                    let events: *const glib::ffi::GPtrArray = unsafe {
                        mem::transmute::<glib::ffi::gpointer, *const glib::ffi::GPtrArray>(
                            glib::gobject_ffi::g_value_get_boxed(events.as_ptr()),
                        )
                    };

                    let len = unsafe { (*events).len as u32 };
                    let mut vec = Vec::new();

                    for i in 0..len {
                        unsafe {
                            let pdata = (*events).pdata as u64;

                            let offset = i as u64
                                * mem::size_of::<*const tsparql::ffi::TrackerNotifierEvent>()
                                    as u64;

                            let mut event_pointer = *((pdata + offset)
                                as *const *mut tsparql::ffi::TrackerNotifierEvent);

                            let event: &mut NotifierEvent =
                                NotifierEvent::from_glib_ptr_borrow_mut(&mut event_pointer);

                            vec.push(event.clone());
                        }
                    }
                    vec
                };

                f(&obj, service, graph, events);
            }),
        )
    }
}
