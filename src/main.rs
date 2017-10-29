use std::cmp::max;
use std::io::{Read, stdin};

extern crate clap;
use clap::{Arg, App};

const ESC: Option<u8> = Some(0x1b);
const TERMINATOR: u8 = 'm' as u8;

pub struct AnsiFilterInner<I> {
    iter: I,
}

impl<I> Iterator for AnsiFilterInner<I>
where
    I: Iterator<Item = u8>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let n = self.iter.next();
        if n.is_none() {
            return None;
        }
        if n == ESC {
            while let Some(b) = self.iter.next() {
                if b == TERMINATOR as u8 {
                    break;
                }
            }
            return self.iter.next();
        } else {
            return n;
        }
    }
}

pub trait AnsiFilter: Iterator {
    fn ansi_filter(self) -> AnsiFilterInner<Self>
    where
        Self: Sized,
    {
        AnsiFilterInner { iter: self }
    }
}

impl<T: ?Sized> AnsiFilter for T
where
    T: Iterator<Item = u8>,
{
}

fn maximum(maximums: &mut Vec<usize>, l: &Vec<usize>) {
    let size = max(maximums.len(), l.len());
    maximums.resize(size, 0);
    for i in 0..l.len() {
        maximums[i] = max(maximums[i], l[i]);
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).expect(
        "read stdin to buffer",
    );
    buffer
}

fn deduce_column_widths(buffer: &str, splitter: &str) -> Vec<usize> {
    let mut column_widths: Vec<usize> = vec![];
    for line in buffer.lines() {
        let segment_lengths: Vec<usize> = line.split(splitter)
            .map(|l| l.bytes().ansi_filter().count())
            .collect();
        maximum(&mut column_widths, &segment_lengths);
    }
    column_widths
}

fn main() {
    let matches = App::new("column")
        .arg(Arg::with_name("split").short("s").takes_value(true))
        .get_matches();

    let splitter = matches.value_of("split").unwrap_or("-");
    let stdin = read_stdin();
    let column_widths = deduce_column_widths(&stdin, splitter);

    for line in stdin.lines() {
        let segments = line.split(splitter);
        let number_of_segments = segments.clone().count();
        for (i, s) in segments.enumerate() {
            let visible_width = s.trim_right().bytes().ansi_filter().count();
            let total_width = s.trim_right().len();
            let width = total_width + (column_widths[i] - visible_width) + 1;

            if i == number_of_segments - 1 {
                print!("{}", s);
            } else {
                print!("{:width$}", s.trim_right(), width = width);
            }
        }
        println!("");
    }
}
