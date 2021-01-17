//! The chained iterator that implements `ExactSizeIterator`.

#![deny(missing_docs)]

/// Chain between one iterator and another.
pub struct ExactChain<A, B> {
    a: Option<A>,
    b: B,
}

impl<A, B> ExactChain<A, B> {
    /// Create a new chained iterator.
    pub fn new(a: A, b: B) -> Self {
        ExactChain { a: Some(a), b }
    }
}

/// Standard iterator extension trait to allow exact chaining.
pub trait IteratorExt<T>: Sized {
    /// Chain this iterator with another.
    fn chain_exact<I: Iterator<Item = T>>(self, other: I) -> ExactChain<Self, I>;
}

impl<J: Iterator> IteratorExt<J::Item> for J {
    fn chain_exact<I: Iterator<Item = J::Item>>(self, other: I) -> ExactChain<Self, I> {
        ExactChain::new(self, other)
    }
}

impl<A, B> Iterator for ExactChain<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    #[inline]
    fn next(&mut self) -> Option<A::Item> {
        match self.a {
            Some(ref mut a) => match a.next() {
                None => {
                    self.a = None;
                    None
                }
                some => some,
            },
            None => self.b.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (b_lower, b_upper) = self.b.size_hint();
        match self.a {
            Some(ref a) => {
                let (a_lower, a_upper) = a.size_hint();
                let lower = a_lower.saturating_add(b_lower);

                let upper = match (a_upper, b_upper) {
                    (Some(x), Some(y)) => x.checked_add(y),
                    _ => None,
                };

                (lower, upper)
            }
            None => (b_lower, b_upper),
        }
    }
}

impl<A: ExactSizeIterator, B: ExactSizeIterator<Item = A::Item>> ExactSizeIterator
    for ExactChain<A, B>
{
    fn len(&self) -> usize {
        match self.a {
            Some(ref a) => a
                .len()
                .checked_add(self.b.len())
                .expect("ExactChain size overflows `usize`"),
            None => self.b.len(),
        }
    }
}

#[test]
fn test_len() {
    assert_eq!((0..10).chain_exact(1..5).len(), 14);
}

#[test]
#[should_panic]
fn test_overflow() {
    (0..usize::MAX).chain_exact(0..10).skip(usize::MAX).len();
}
