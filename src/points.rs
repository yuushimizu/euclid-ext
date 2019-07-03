use crate::point2d_range_iterator::{Point2DRangeIterator, PointRangeIteratorPrimitive};
use euclid;
use std::ops;

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
impl<T: PointRangeIteratorPrimitive, U> Points for euclid::TypedRect<T, U> {
    type Iter = Point2DRangeIterator<T, U>;

    fn points(self) -> Self::Iter {
        Point2DRangeIterator::new(self.origin..self.origin + self.size)
    }
}

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
impl<T: PointRangeIteratorPrimitive, U> Points for euclid::TypedBox2D<T, U> {
    type Iter = Point2DRangeIterator<T, U>;

    fn points(self) -> Self::Iter {
        Point2DRangeIterator::new(self.min..self.max)
    }
}

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
impl<T: PointRangeIteratorPrimitive, U> Points for ops::Range<euclid::TypedPoint2D<T, U>> {
    type Iter = Point2DRangeIterator<T, U>;

    fn points(self) -> Self::Iter {
        Point2DRangeIterator::new(self)
    }
}