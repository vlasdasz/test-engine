use rapier2d::{
    crossbeam::channel::{unbounded, Receiver},
    geometry::{CollisionEvent, ContactForceEvent},
    prelude::ChannelEventCollector,
};

pub(crate) struct EventHandler {
    pub(crate) _contact:     Receiver<ContactForceEvent>,
    pub(crate) intersection: Receiver<CollisionEvent>,
    pub(crate) handler:      ChannelEventCollector,
}

impl Default for EventHandler {
    #[allow(clippy::used_underscore_binding)]
    fn default() -> Self {
        let (contact_send, _contact) = unbounded();
        let (intersection_send, intersection) = unbounded();
        let handler = ChannelEventCollector::new(intersection_send, contact_send);
        Self {
            _contact,
            intersection,
            handler,
        }
    }
}
