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
