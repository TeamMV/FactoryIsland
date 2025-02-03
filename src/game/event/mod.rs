use std::collections::VecDeque;
use crate::game::event::controls::CameraMoveEvent;
use crate::game::event::world::{ChunkGenerateEvent, ChunkLoadEvent};

pub mod world;
pub mod controls;

pub enum GameEvent {
    ChunkLoad(ChunkLoadEvent),
    ChunkGenerate(ChunkGenerateEvent),
    CameraMove(CameraMoveEvent)
}

pub struct EventQueue {
    queued: VecDeque<GameEvent>
}

impl EventQueue {
    fn new() -> Self {
        Self {
            queued: VecDeque::new(),
        }
    }

    pub fn dispatch(&mut self, event: GameEvent) {
        self.queued.push_back(event);
    }

    pub fn iter(self) -> impl Iterator<Item=GameEvent> {
        self.queued.into_iter()
    }
}

pub struct EventDispatcher {
    receivers: Vec<fn(&GameEvent, &mut EventQueue) -> bool>,
    game_receiver: Box<dyn FnMut(&GameEvent)>
}

impl EventDispatcher {
    pub fn new(game_receiver: Box<dyn FnMut(&GameEvent)>) -> Self {
        Self { receivers: vec![], game_receiver }
    }

    pub fn add_receiver(&mut self, receiver: fn(&GameEvent, &mut EventQueue) -> bool) {
        self.receivers.push(receiver);
    }
}

impl EventDispatcher {
    pub fn dispatch(&mut self, event: GameEvent) {
        let mut cancelled = false;
        let mut queue = EventQueue::new();
        for receiver in &self.receivers {
            let cancel = receiver(&event, &mut queue);
            cancelled |= cancel;
        }
        if !cancelled {
            (self.game_receiver)(&event)
        }
        for dispatched in queue.iter() {
            (self.game_receiver)(&dispatched);
            self.dispatch(dispatched);
        }
    }
}