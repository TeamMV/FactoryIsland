use std::collections::{vec_deque, VecDeque};
use std::vec;
use crate::game::world::chunk::Chunk;
use crate::game::world::generator::GeneratorPipeline;

pub enum Event<'a> {
    ChunkLoad(ChunkLoadEvent),
    ChunkGenerate(ChunkGenerateEvent<'a>)
}

pub struct ChunkLoadEvent {
    pub x: i32,
    pub z: i32,
    pub cancelled: bool,
    pub forced: bool
}

pub struct ChunkGenerateEvent<'a> {
    pub chunk: &'a mut Chunk,
    pub pipeline: GeneratorPipeline,
    pub cancelled: bool,
    pub forced: bool
}

// Generated

pub trait LmaoEnumHandler {
    fn filter(&self, event: &mut Event) -> bool { true }

    fn handle(&mut self, event: &mut Event) {
        if !self.filter(event) { return; }
        match event {
            Event::ChunkLoad(event) => self.handle_chunk_load(event),
            Event::ChunkGenerate(event) => self.handle_chunk_generate(event)
        }
    }

    fn handle_chunk_load(&mut self, event: &mut ChunkLoadEvent) {}

    fn handle_chunk_generate(&mut self, event: &mut ChunkGenerateEvent) {}
}

pub struct EventDispatcher<'a> {
    handlers: Vec<Box<dyn LmaoEnumHandler>>,
    queue: VecDeque<Event<'a>>
}

unsafe impl Send for EventDispatcher<'_> {}
unsafe impl Sync for EventDispatcher<'_> {}

impl<'a> EventDispatcher<'a> {
    pub fn new() -> Self {
        Self { handlers: vec![], queue: VecDeque::new() }
    }

    pub fn dispatch(&mut self, event: Event) {
        self.queue.push_back(event);
        let event = self.queue.back_mut().expect("Element must exist as it was just pushed");
        self.handlers.iter_mut().for_each(|handler| handler.handle(event));
    }

    pub fn pump(&mut self) -> EventPump<'_> {
        let drain = self.queue.drain(..);
        EventPump {
            dispatcher: self,
            events: drain,
        }
    }

    pub fn add_event_handler(&mut self, handler: impl LmaoEnumHandler + 'static) {
        self.handlers.push(Box::new(handler));
    }
}

pub struct EventPump<'a> {
    dispatcher: &'a mut EventDispatcher<'a>,
    events: vec_deque::Drain<'a, Event<'a>>
}

impl<'a> Iterator for EventPump<'a> {
    type Item = (&'a mut EventDispatcher<'a>, Event<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.events.next() {
            Some((self.dispatcher, next))
        } else {
            None
        }
    }
}

/*

mod1    ->    *a +=1;
mod2    ->    if *a == 1 { do_fancy(); }

//game

vec[mod1, mod2]

 */