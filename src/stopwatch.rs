use std::time::Instant;
use std::{io::Write, time::Duration};

use crate::input::{get_event, TIMEOUT};
use crate::prelude::*;
use crate::terminal::running_color;
use crate::{format::format_duration, input::Command};
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    queue,
    style::{Print, Stylize},
    terminal::{Clear, ClearType},
};

pub struct Stopwatch {
    start_time: Option<Instant>,
    elapsed_before: Duration,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            start_time: Some(Instant::now()),
            elapsed_before: Duration::ZERO,
        }
    }
}

impl Stopwatch {
    pub fn new(start_time: Option<Instant>, elapsed_before: Duration) -> Self {
        Self {
            start_time,
            elapsed_before,
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(start_time) => self.elapsed_before + start_time.elapsed(),
            None => self.elapsed_before,
        }
    }

    pub fn started(&self) -> bool {
        if matches!(self.start_time, None) {
            false
        } else {
            true
        }
    }

    pub fn start(&mut self) {
        if matches!(self.start_time, None) {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(start_time) = self.start_time {
            self.elapsed_before += start_time.elapsed();
            self.start_time = None;
        }
    }

    pub fn toggle(&mut self) {
        match self.start_time {
            Some(start_time) => {
                self.elapsed_before += start_time.elapsed();
                self.start_time = None;
            }
            None => {
                self.start_time = Some(Instant::now());
            }
        }
    }
}

pub fn stopwatch(out: &mut impl Write, start_time: Duration) -> Result<()> {
    let mut counter = Stopwatch::new(Some(Instant::now()), start_time);
    loop {
        queue!(
            out,
            MoveTo(0, 0),
            Clear(ClearType::All),
            Print("Stopwatch"),
            MoveToNextLine(1),
            Print(format_duration(&counter.elapsed()).with(running_color(counter.started()))),
            MoveToNextLine(1),
            Print("[Q]: quit, [Space]: pause/resume"),
        )?;
        out.flush()?;
        if let Some(cmd) = get_event(TIMEOUT)?.map(Command::from) {
            match cmd {
                Command::Quit => break,
                Command::Pause => counter.stop(),
                Command::Resume => counter.start(),
                Command::Toggle | Command::Enter => counter.toggle(),
                _ => (),
            }
        }
    }

    Ok(())
}