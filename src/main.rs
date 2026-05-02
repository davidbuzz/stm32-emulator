// SPDX-License-Identifier: GPL-3.0-or-later

mod config;
mod emulator;
mod gdb;
mod util;
mod peripherals;
mod ext_devices;
mod system;
mod framebuffers;

use std::io::{self, prelude::*};
use std::sync::atomic::Ordering::Relaxed;
use clap::Parser;
use anyhow::{Result, Context};
use env_logger::fmt::WriteStyle;
use log::LevelFilter;

use config::Config;
use emulator::run_emulator;
use util::read_file_str;


#[macro_use]
extern crate log;

/// STM32 Emulator
#[derive(Parser, Debug)]
pub struct Args {
    /// Config file
    #[clap(default_value = "config.yaml")]
    config: String,

    /// Verbosity. Can be repeated. -vvvv is the maximum.
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Maximum number of instructions to execute
    #[clap(short, long)]
    max_instructions: Option<u64>,

    /// Stop emulation when pc reaches this address
    #[clap(short, long, parse(try_from_str=clap_num::maybe_hex))]
    stop_addr: Option<u32>,

    /// Stop emulation when the program reaches a busy loop
    #[clap(short, long)]
    busy_loop_stop: bool,

    /// Colorize output
    #[clap(short, long, arg_enum, default_value="auto")]
    color: Color,

    /// Run pending interrupts every N instructions
    /// Shorter is more correct, but is slower.
    #[clap(short, long, default_value="1")]
    interrupt_period: u32,

    /// Dump stack at the end. Parameter is the number of words to print
    #[clap(short, long)]
    dump_stack: Option<usize>,

    /// Print only ArduPilot console output (USB CDC EP1). Suppresses all emulator log noise.
    #[clap(long)]
    console_only: bool,

    /// Enable GDB RSP server on the given TCP port (e.g. --gdb 3333).
    /// The emulator halts at the reset vector and waits for a GDB connection.
    #[clap(long)]
    gdb: Option<u16>,
}

#[derive(clap::ArgEnum, Clone, Copy, Debug)]
enum Color {
    Auto,
    Always,
    Never,
}

impl std::convert::From<Color> for WriteStyle {
    fn from(c: Color) -> Self {
        match c {
            Color::Always => WriteStyle::Always,
            Color::Never => WriteStyle::Never,
            Color::Auto => WriteStyle::Auto,
        }
    }
}

static mut VERBOSE: u8 = 0;
static mut CONSOLE_ONLY: bool = false;

pub fn verbose() -> u8 {
    unsafe { VERBOSE }
}

pub fn console_only() -> bool {
    unsafe { CONSOLE_ONLY }
}

fn init_logging(args: &Args) {
    unsafe { VERBOSE = args.verbose };
    unsafe { CONSOLE_ONLY = args.console_only };

    let lf = if args.console_only {
        LevelFilter::Off
    } else {
        match args.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };

    struct FlushWriter(io::Stdout);
    impl io::Write for FlushWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let n = self.0.write(buf)?;
            self.0.flush()?;
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> { self.0.flush() }
    }

    static mut LAST_NUM_INSTRUCTIONS: u64 = 0;

    env_logger::Builder::new()
        .filter_level(lf)
        .write_style(args.color.into())
        .target(env_logger::Target::Pipe(Box::new(FlushWriter(io::stdout()))))
        .format(|buf, record| {
            use env_logger::fmt::Color;
            let num_instructions = emulator::NUM_INSTRUCTIONS.load(Relaxed);
            //let delta_instructions = num_instructions - unsafe { LAST_NUM_INSTRUCTIONS };
            unsafe { LAST_NUM_INSTRUCTIONS = num_instructions };
            let pc = unsafe { emulator::LAST_INSTRUCTION.0 };

            let mut style = buf.style();
            let level = match record.level() {
                log::Level::Error => style.set_color(Color::Red).set_intense(true).value("ERROR"),
                log::Level::Warn =>  style.set_color(Color::Yellow).set_intense(true).value("WARN "),
                log::Level::Info =>  style.set_color(Color::Green).set_intense(true).value("INFO "),
                log::Level::Debug => style.set_color(Color::Cyan).set_intense(true).value("DEBUG"),
                log::Level::Trace => style.set_color(Color::Blue).set_intense(true).value("TRACE"),
            };

            let mut style = buf.style();
            style.set_color(Color::Black).set_intense(true);
            //let header = format!("[tsc={:08} dtsc=+{:08} pc=0x{:08x}]", num_instructions, delta_instructions, pc);
            let header = format!("[clk={:08} pc=0x{:08x}]", num_instructions, pc);
            let header = style.value(header);

            writeln!(buf, "{} {} {}", header, level, record.args())
        })
        .init();
}

fn main() -> Result<()> {
    let args = Args::parse();
    init_logging(&args);

    if args.console_only {
        eprintln!("console-only: streaming USB CDC EP1 bytes (other emulator logs suppressed)");
    }

    let config: Config = serde_yaml::from_str(&read_file_str(&args.config)?)
        .with_context(|| format!("Failed to parse {}", args.config))?;

    let device = svd_parser::parse(&read_file_str(&config.cpu.svd)?)
        .with_context(|| format!("Failed to parse {}", config.cpu.svd))?;

    let result = run_emulator(config, device, args);
    if console_only() {
        eprintln!("console-only: emulation finished");
    }
    result
}
