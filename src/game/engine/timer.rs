use super::events::{Events, EventEnum};
use std::hash::Hash;

pub enum TimerEvent {
    TimerStart,
    TimerStep,
    TimerPause,
    TimerResume,
    TimerEnd,
}

impl EventEnum for TimerEvent {
    fn as_i32(&self) -> i32 {
        return match self {
            TimerEvent::TimerStart => 0,
            TimerEvent::TimerStep => 1,
            TimerEvent::TimerPause => 2,
            TimerEvent::TimerResume => 3,
            TimerEvent::TimerEnd => 4,
        };
    }

    fn as_str(&self) -> &'static str {
        match self {
            TimerEvent::TimerStart => "TimerStart",
            TimerEvent::TimerStep => "TimerStep",
            TimerEvent::TimerPause => "TimerPause",
            TimerEvent::TimerResume => "TimerResume",
            TimerEvent::TimerEnd => "TimerEnd",
        }
    }
}

impl PartialEq for TimerEvent {
    fn eq(&self, other: &Self) -> bool {
        return self.as_i32() == other.as_i32();
    }

    fn ne(&self, other: &Self) -> bool {
        return self.as_i32() != other.as_i32();
    }
}

// Eq is what we call a marker trait: it has no method on its own, it is just a way for the programmer to express that the struct verifies a certain property.
impl Eq for TimerEvent {}

impl Hash for TimerEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.as_i32());
        state.finish();
    }
}

pub struct Timer {
    time: i32,     // eleapsedTime since start
    end: i32,      // end time
    paused: bool,  // is_paused
    restart: bool, // should restart ?
    events: Events<TimerEvent>,
}

impl Timer {
    pub fn new(end: i32, restart: bool, autostart: bool) -> Timer {
        Timer {
            time: 0,
            end,
            paused: !autostart,
            restart,
            events: Events::new(),
        }
    }

    pub fn get_events(&mut self) -> &mut Events<TimerEvent> {
        &mut self.events
    }
    pub fn step(&mut self) {
        if self.paused {
            return;
        }

        self.time += 1;
        if self.time == 1 {
            self.events.trigger(TimerEvent::TimerStart);
        }
        self.events.trigger(TimerEvent::TimerStep);
        if self.time >= self.end {
            self.reset();
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
        self.events.trigger(TimerEvent::TimerPause);
    }

    pub fn resume(&mut self) {
        self.paused = false;
        self.events.trigger(TimerEvent::TimerResume);
    }

    pub fn reset(&mut self) {
        self.time = 0;
        if !self.restart {
            self.paused = true; // hard hack to force stop
        }
        self.events.trigger(TimerEvent::TimerEnd);
    }

    pub fn restart(&mut self) {
        self.paused = false;
    }

    pub fn time_left(&self) -> i32 {
        self.end - self.time
    }

    pub fn set_time(&mut self, time: i32)
    {
        self.time = time;
    }

    pub fn time(&self) -> i32
    {
        self.time
    }
}