use crate::point_range_iterator::{PointRangeIterator, PointRangeIteratorItem};
use crate::to_point_range::ToPointRange;

pub trait Points {
    type Iter: Iterator;

    fn points(self) -> Self::Iter;
}

/// # Examples
/// ```
/// # use euclid::{Rect, Point2D, Size2D};
/// # use euclid_ext::Points;
/// let rect = Rect::new(Point2D::new(10, 20), Size2D::new(3, 2));
/// assert_eq!(
///     vec![
///         Point2D::new(10, 20), Point2D::new(11, 20), Point2D::new(12, 20),
///         Point2D::new(10, 21), Point2D::new(11, 21), Point2D::new(12, 21)
///     ],
///     rect.points().collect::<Vec<_>>());
/// ```
///

/// # Examples
/// ```
/// # use euclid::{Box2D, Point2D};
/// # use euclid_ext::Points;
/// let box2d = Box2D::new(Point2D::new(10, 20), Point2D::new(13, 22));
/// assert_eq!(
///     vec![
///         Point2D::new(10, 20), Point2D::new(11, 20), Point2D::new(12, 20),
///         Point2D::new(10, 21), Point2D::new(11, 21), Point2D::new(12, 21)
///     ],
///     box2d.points().collect::<Vec<_>>());
/// ```
///

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::Points;
/// let range = Point2D::new(10, 20)..Point2D::new(13, 22);
/// assert_eq!(
///     vec![
///         Point2D::new(10, 20), Point2D::new(11, 20), Point2D::new(12, 20),
///         Point2D::new(10, 21), Point2D::new(11, 21), Point2D::new(12, 21)
///     ],
///     range.points().collect::<Vec<_>>());
/// ```
impl<T: ToPointRange> Points
    for T where T::Point: PointRangeIteratorItem
{
    type Iter = PointRangeIterator<T::Point>;

    fn points(self) -> Self::Iter {
        PointRangeIterator::new(self.to_point_range())
    }
}
