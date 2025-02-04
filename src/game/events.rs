use std::collections::VecDeque;

pub enum LmaoEnum {
    ChunkLoad(ChunkLoadEvent),
}

pub struct ChunkLoadEvent {
    pub x: i32,
    pub z: i32,
    pub cancelled: bool,
}

// Generated

pub trait LmaoEnumHandler {
    fn filter(&self, event: &mut LmaoEnum) -> bool { true }

    fn handle(&mut self, event: &mut LmaoEnum) {
        if !self.filter(event) { return; }
        match event {
            LmaoEnum::ChunkLoad(event) => self.handle_chunk_load(event),
        }
    }

    fn handle_chunk_load(&mut self, event: &mut ChunkLoadEvent) {}
}

pub struct LmaoEnumDispatcher {
    handlers: Vec<Box<dyn LmaoEnumHandler>>,
    queue: VecDeque<LmaoEnum>
}

impl LmaoEnumDispatcher {
    pub fn new() -> Self {
        Self { handlers: vec![], queue: VecDeque::new() }
    }

    pub fn dispatch(&mut self, event: LmaoEnum) {
        self.queue.push_back(event);
        let event = self.queue.back_mut().expect("Element must exist as it was just pushed");
        self.handlers.iter_mut().for_each(|handler| handler.handle(event));
    }

    pub fn poll(&mut self) -> impl Iterator<Item=LmaoEnum> + use<'_> {
        self.queue.drain(..)
    }

    pub fn add_event_handler(&mut self, handler: impl LmaoEnumHandler + 'static) {
        self.handlers.push(Box::new(handler));
    }
}

/*

mod1    ->    *a +=1;
mod2    ->    if *a == 1 { do_fancy(); }

//game

vec[mod1, mod2]

 */