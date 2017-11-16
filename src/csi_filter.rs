use std::collections::VecDeque;

const ESC: Option<u8> = Some(0x1b);
const CSI_START: Option<u8> = Some('[' as u8);

pub struct CsiFilter<I>
where
    I: Iterator,
{
    iter: I,
    peeks: VecDeque<Option<I::Item>>,
}

#[inline]
fn is_terminator(byte: u8) -> bool {
    byte >= 0x40 && byte <= 0x7e
}

impl<I> Iterator for CsiFilter<I>
where
    I: Iterator<Item = u8>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        // Drain the peek deque first, if it's not empty when we reach this
        // point we know we should drain it.
        if !self.peeks.is_empty() {
            return self.peeks.pop_front().unwrap();
        }
        let mut n = self.iter.next();
        loop {
            if n.is_none() {
                return None;
            } else if n == ESC {
                // Push the escape onto the peek deque
                self.peeks.push_back(n);
                // Peek the next character
                let peek = self.iter.next();
                if peek == CSI_START {
                    // We're in a valid ANSI escape sequence. The peek deque
                    // contains stuff that should be filtered, so just clean it
                    // and proceed to the loop that consumes the rest of the
                    // escape sequence.
                    self.peeks.clear();
                } else {
                    // We're not in a valid ANSI escape sequence and will need
                    // to yield the peeked values in the iteration. First
                    // append the peeked value onto the deque, then pop the
                    // first item off and return it. In the next call to next()
                    // we'll pop stuff off the peeked deque first, before
                    // continuing to consume from the inner iterator.
                    self.peeks.push_back(peek);
                    return self.peeks.pop_front().unwrap();
                }
                // Consume the rest of the ANSI escape sequence up to and
                // including the terminator byte.
                while let Some(b) = self.iter.next() {
                    if is_terminator(b) {
                        break;
                    }
                }
            } else {
                return n;
            }
            n = self.iter.next();
        }
    }
}

pub trait CsiFilterable: Iterator {
    fn filter_csi(self) -> CsiFilter<Self>
    where
        Self: Sized,
    {
        CsiFilter {
            iter: self,
            peeks: VecDeque::new(),
        }
    }
}

impl<T: ?Sized> CsiFilterable for T
where
    T: Iterator<Item = u8>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_ansi_escape() {
        let visible: Vec<u8> = "hello".bytes().collect();
        let filtered: Vec<u8> = "hello".bytes().filter_csi().collect();
        assert_eq!(visible, filtered);
    }

    #[test]
    fn single_ansi_escape() {
        let visible: Vec<u8> = "hello".bytes().collect();
        let filtered: Vec<u8> = "hel\x1b[32mlo".bytes().filter_csi().collect();
        assert_eq!(visible, filtered);
    }

    #[test]
    fn adjacent_ansi_escapes() {
        let visible: Vec<u8> = "ab".bytes().collect();
        let filtered: Vec<u8> = "\x1b[32ma\x1b[m\x1b[31mb\x1b[m"
            .bytes()
            .filter_csi()
            .collect();
        assert_eq!(visible, filtered);
    }

    #[test]
    fn string_ends_in_the_middle_of_an_ansi_escape() {
        let visible: Vec<u8> = "ab".bytes().collect();
        let filtered: Vec<u8> = "ab\x1b[3".bytes().filter_csi().collect();
        assert_eq!(visible, filtered);
    }

    #[test]
    fn string_contains_escape_but_not_a_valid_ansi_escape_sequence() {
        let visible: Vec<u8> = "ab\x1b(3".bytes().collect();
        let filtered: Vec<u8> = "ab\x1b(3".bytes().filter_csi().collect();
        assert_eq!(visible, filtered);
    }
}
