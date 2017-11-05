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
        let mut n = self.iter.next();
        loop {
            if n.is_none() {
                return None;
            } else if n == ESC {
                while let Some(b) = self.iter.next() {
                    if b == TERMINATOR as u8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_ansi_escape() {
        let visible: Vec<u8> = "hello".bytes().collect();
        let filtered: Vec<u8> = "hello".bytes().ansi_filter().collect();
        assert_eq!(visible, filtered);
    }

    #[test]
    fn single_ansi_escape() {
        let visible: Vec<u8> = "hello".bytes().collect();
        let filtered: Vec<u8> = "hel\x1b[32mlo".bytes().ansi_filter().collect();
        assert_eq!(visible, filtered);
    }

    #[test]
    fn adjacent_ansi_escapes() {
        let visible: Vec<u8> = "ab".bytes().collect();
        let filtered: Vec<u8> = "\x1b[32ma\x1b[m\x1b[31mb\x1b[m"
            .bytes()
            .ansi_filter()
            .collect();
        assert_eq!(visible, filtered);
    }

    #[test]
    fn string_ends_in_the_middle_of_an_ansi_escape() {
        let visible: Vec<u8> = "ab".bytes().collect();
        let filtered: Vec<u8> = "ab\x1b[3".bytes().ansi_filter().collect();
        assert_eq!(visible, filtered);
    }
}
