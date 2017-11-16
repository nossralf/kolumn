use std::cmp::max;
use std::io::{Read, stdin, stdout, Write};

extern crate clap;
use clap::{App, Arg};

mod csi_filter;
use csi_filter::CsiFilterable;

fn maximum(maximums: &mut Vec<usize>, l: &[usize]) {
    let size = max(maximums.len(), l.len());
    maximums.resize(size, 0);
    for i in 0..l.len() {
        maximums[i] = max(maximums[i], l[i]);
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).expect(
        "Reading stdin to buffer",
    );
    buffer
}

fn deduce_column_widths(buffer: &str, splitter: &str) -> Vec<usize> {
    let mut column_widths: Vec<usize> = vec![];
    for line in buffer.lines() {
        let segment_lengths: Vec<usize> = line.split(splitter)
            .map(|l| l.bytes().filter_csi().count())
            .collect();
        maximum(&mut column_widths, &segment_lengths);
    }
    column_widths
}

fn write_output(buffer: &str, splitter: &str, column_widths: &[usize], out: &mut Write) {
    for line in buffer.lines() {
        let segments = line.split(splitter);
        let number_of_segments = segments.clone().count();
        for (i, s) in segments.enumerate() {
            let visible_width = s.bytes().filter_csi().count();
            let width = s.len() + (column_widths[i] - visible_width) + 1;

            if i == number_of_segments - 1 {
                write!(out, "{}", s).expect("Writing to stdout");
            } else {
                write!(out, "{:width$}", s, width = width).expect("Writing to stdout");
            }
        }
        writeln!(out, "").expect("Writing to stdout");
    }
}

fn main() {
    let matches = App::new("column")
        .arg(Arg::with_name("split").short("s").takes_value(true))
        .get_matches();

    let splitter = matches.value_of("split").unwrap_or("-");
    let stdin = read_stdin();
    let column_widths = deduce_column_widths(&stdin, splitter);

    let stdout = stdout();
    let mut out = stdout.lock();
    write_output(&stdin, splitter, &column_widths, &mut out);
}
