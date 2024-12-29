use std::collections::HashMap;
use std::hash::Hash;
pub trait EventEnum {
    fn as_i32(&self) -> i32;
    fn as_str(&self) -> &'static str; // we enforce this to make debugging easier
}

pub struct Events<T: EventEnum + Eq + Hash> {
    listeners: HashMap<T, Vec<Box<dyn Fn()>>>,
}

impl<T: EventEnum + Eq + Hash> Events<T>
where
    T: EventEnum,
{
    pub fn new() -> Events<T> {
        Events {
            listeners: HashMap::new(),
        }
    }

    pub fn listen_legacy(&mut self, etype: T, f: fn()) {
        self.listeners
            .entry(etype)
            .or_insert_with(Vec::new)
            .push(Box::new(f));
    }

    pub fn listen(&mut self, etype: T, f: Box<dyn Fn()>) {
        self.listeners.entry(etype).or_insert_with(Vec::new).push(f);
    }

    pub fn trigger(&mut self, etype: T) {
        for listener in self.listeners.entry(etype).or_insert_with(Vec::new) {
            listener();
        }
    }
}