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
