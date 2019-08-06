use ansi_term::{
    Colour::{Blue, Green, Red, Yellow},
    Style,
};
use log::{Level, Log, Metadata, Record};

pub static LOGGER: Logger = Logger;

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                Level::Error => Style::new().bold().fg(Red).paint("[error]"),
                Level::Warn => Style::new().bold().fg(Yellow).paint("[warn]"),
                Level::Info => Style::new().bold().fg(Blue).paint("[info]"),
                Level::Debug => Style::new().bold().fg(Green).paint("[debug]"),
                Level::Trace => Style::new().bold().fg(Green).paint("[trace]"),
            };
            println!("{} {}", level, record.args());
        }
    }
    fn flush(&self) {}
}
