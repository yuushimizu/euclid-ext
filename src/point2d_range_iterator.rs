use crate::type_alias::{Point2D, Point2DRange};
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

pub struct Point2DRangeIterator<T: PointRangeIteratorPrimitive, U> {
    range: Point2DRange<T, U>,
    current: Point2D<T, U>,
}

impl<T: PointRangeIteratorPrimitive, U> Point2DRangeIterator<T, U> {
    pub fn new(range: Point2DRange<T, U>) -> Self {
        let current = range.start;
        Self { range, current }
    }
}

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::Point2DRangeIterator;
/// let mut i = Point2DRangeIterator::new(Point2D::new(10, 20)..Point2D::new(12, 22));
/// assert_eq!(Some(Point2D::new(10, 20)), i.next());
/// assert_eq!(Some(Point2D::new(11, 20)), i.next());
/// assert_eq!(Some(Point2D::new(10, 21)), i.next());
/// assert_eq!(Some(Point2D::new(11, 21)), i.next());
/// assert_eq!(None, i.next());
/// ```
impl<T: PointRangeIteratorPrimitive, U> Iterator for Point2DRangeIterator<T, U> {
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