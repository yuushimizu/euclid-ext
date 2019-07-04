use crate::type_alias::Point2D;
use euclid;
use std::ops;

pub trait PointRangeIteratorPrimitive:
    Copy + PartialOrd + ops::Add<Output = Self> + euclid::num::One
{
}

impl<T: Copy + PartialOrd + ops::Add<Output = Self> + euclid::num::One> PointRangeIteratorPrimitive
    for T
{
}

pub struct PointRangeIterator<T> {
    range: ops::Range<T>,
    current: T,
}

impl<T> PointRangeIterator<T> {
    pub fn new(range: ops::Range<T>) -> Self
    where
        T: Clone,
    {
        let current = range.start.clone();
        Self { range, current }
    }
}

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::PointRangeIterator;
/// let mut i = PointRangeIterator::new(Point2D::new(10, 20)..Point2D::new(12, 22));
/// assert_eq!(Some(Point2D::new(10, 20)), i.next());
/// assert_eq!(Some(Point2D::new(11, 20)), i.next());
/// assert_eq!(Some(Point2D::new(10, 21)), i.next());
/// assert_eq!(Some(Point2D::new(11, 21)), i.next());
/// assert_eq!(None, i.next());
/// ```
impl<T: PointRangeIteratorPrimitive, U> Iterator for PointRangeIterator<Point2D<T, U>> {
    type Item = Point2D<T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current.y < self.range.end.y {
            if self.current.x < self.range.end.x {
                let result = Some(self.current);
                self.current.x = self.current.x + T::one();
                return result;
            }
            self.current = Point2D::new(self.range.start.x, self.current.y + T::one());
        }
        None
    }
}