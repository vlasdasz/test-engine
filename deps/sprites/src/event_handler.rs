use crossbeam::channel::Receiver;
use rapier2d::{
    geometry::{CollisionEvent, ContactForceEvent},
    prelude::ChannelEventCollector,
};

pub(crate) struct EventHandler {
    pub(crate) _contact:     Receiver<ContactForceEvent>,
    pub(crate) intersection: Receiver<CollisionEvent>,
    pub(crate) handler:      ChannelEventCollector,
}

impl Default for EventHandler {
    fn default() -> Self {
        let (contact_send, _contact) = crossbeam::channel::unbounded();
        let (intersection_send, intersection) = crossbeam::channel::unbounded();
        let handler = ChannelEventCollector::new(intersection_send, contact_send);
        Self {
            _contact,
            intersection,
            handler,
        }
    }
}
