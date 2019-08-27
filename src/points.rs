use crate::point_range_iterator::{PointRangeIterator, PointRangeIteratorItem};
use crate::to_point_range::ToPointRange;

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::Points;
/// enum Space {}
/// let range = Point2D::<i64, Space>::new(10, 20)..Point2D::<i64, Space>::new(13, 22);
/// assert_eq!(
///     vec![
///         Point2D::new(10, 20), Point2D::new(11, 20), Point2D::new(12, 20),
///         Point2D::new(10, 21), Point2D::new(11, 21), Point2D::new(12, 21)
///     ],
///     range.points().collect::<Vec<_>>());
/// ```
pub trait Points {
    type Iter: Iterator;

    fn points(self) -> Self::Iter;
}

impl<T: ToPointRange> Points for T
where
    T::Point: PointRangeIteratorItem,
{
    type Iter = PointRangeIterator<T::Point>;

    fn points(self) -> Self::Iter {
        PointRangeIterator::new(self.to_point_range())
    }
}
