// Copyright (c) Aptos Foundation
// Licensed pursuant to the Innovation-Enabling Source Code License, available at https://github.com/aptos-labs/aptos-core/blob/main/LICENSE

//! Minimal code-emission helper for generating Move script source text.
//!
//! Replaces the dependency on `move_model::code_writer::CodeWriter` which pulled in
//! the entire Move compiler chain.

use std::fmt::Write;

/// A lightweight string builder with indentation tracking.
pub(crate) struct CodeEmitter {
    output: String,
    indent: usize,
    at_line_start: bool,
}

impl CodeEmitter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            at_line_start: true,
        }
    }

    pub fn indent(&mut self) {
        self.indent += 4;
    }

    pub fn unindent(&mut self) {
        assert!(self.indent >= 4, "unindent underflow");
        self.indent -= 4;
    }

    pub fn emit(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        if self.at_line_start {
            for _ in 0..self.indent {
                self.output.push(' ');
            }
            self.at_line_start = false;
        }
        self.output.push_str(s);
    }

    pub fn emit_line(&mut self, s: &str) {
        self.emit(s);
        self.output.push('\n');
        self.at_line_start = true;
    }

    pub fn emit_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        if self.at_line_start {
            for _ in 0..self.indent {
                self.output.push(' ');
            }
            self.at_line_start = false;
        }
        let _ = self.output.write_fmt(args);
    }

    pub fn emitln_fmt(&mut self, args: std::fmt::Arguments<'_>) {
        self.emit_fmt(args);
        self.output.push('\n');
        self.at_line_start = true;
    }

    pub fn into_output(self) -> String {
        self.output
    }
}

macro_rules! emit {
    ($target:expr, $s:expr) => {
        $target.emit($s)
    };
    ($target:expr, $s:expr, $($arg:expr),+ $(,)?) => {
        $target.emit_fmt(format_args!($s, $($arg),+))
    };
}

macro_rules! emitln {
    ($target:expr) => {
        $target.emit_line("")
    };
    ($target:expr, $s:expr) => {
        $target.emit_line($s)
    };
    ($target:expr, $s:expr, $($arg:expr),+ $(,)?) => {
        $target.emitln_fmt(format_args!($s, $($arg),+))
    };
}

pub(crate) use emit;
pub(crate) use emitln;
