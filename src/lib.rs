use std::iter::Peekable;

/// This `struct` is created by the [`mark_last`] method on [`MarkLastIterator`]. See its
/// documentation for more.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct MarkLast<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator> MarkLast<I>
where
    I: Iterator,
{
    fn new(iter: I) -> MarkLast<I> {
        MarkLast {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator> Iterator for MarkLast<I> {
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<(bool, I::Item)> {
        let val = self.iter.next()?;
        let last = self.iter.peek().is_none();
        Some((last, val))
    }
}

pub trait MarkLastIterator<I: Iterator> {
    /// Creates an iterator which gives the next value as well as a boolean indicating if this is
    /// the last value of the iterator.
    ///
    /// The iterator returned yields pairs `(b, val)`, where `b` is true if this is the last value
    /// and `val` is the value returned by the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::mark_last::MarkLastIterator;
    /// let in_data = vec![1, 2, 3, 5, 99];
    /// let out_data: Vec<_> = in_data.into_iter().mark_last().collect();
    /// assert_eq!(
    ///     out_data,
    ///     vec![(false, 1), (false, 2), (false, 3), (false, 5), (true, 99)]
    /// );
    /// ```
    fn mark_last(self) -> MarkLast<I>;
}

impl<I: Iterator> MarkLastIterator<I> for I {
    fn mark_last(self) -> MarkLast<Self> {
        MarkLast::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::MarkLastIterator;

    #[test]
    fn marks_last() {
        let in_data = vec![1, 2, 3, 5, 99];
        let out_data: Vec<_> = in_data.into_iter().mark_last().collect();
        assert_eq!(
            out_data,
            vec![(false, 1), (false, 2), (false, 3), (false, 5), (true, 99)]
        );
    }

    #[test]
    fn empty_collection() {
        let in_data: [i32; 0] = [];
        let out_data: Vec<_> = in_data.iter().mark_last().collect();
        assert_eq!(out_data, vec![]);
    }

    #[test]
    fn marks_last_length_one() {
        let in_data = vec![3];
        let out_data: Vec<_> = in_data.into_iter().mark_last().collect();
        assert_eq!(out_data, vec![(true, 3)]);
    }

    #[test]
    fn marks_nothing_infinite() {
        let in_data = 0..;
        assert!(in_data.mark_last().take(1_000_000).all(|(last, _)| !last));
    }
}
