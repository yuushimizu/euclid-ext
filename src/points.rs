use crate::point2d_range_iterator::{Point2DRangeIterator, Point2DRangeIteratorPrimitive};
use euclid;
use std::ops;

pub trait Point {}

impl<T, U> Point for euclid::TypedPoint2D<T, U> {}

impl<T, U> Point for euclid::TypedPoint3D<T, U> {}

pub trait Points
where
    <Self::Iter as Iterator>::Item: Point,
{
    type Iter: Iterator;

    fn points(self) -> Self::Iter;
}

/// # Examples
/// ```
/// # use euclid::{Rect, Point2D, Size2D};
/// # use euclid_ext::Points;
/// assert_eq!(
///     vec![
///         Point2D::new(10, 20), Point2D::new(11, 20), Point2D::new(12, 20),
///         Point2D::new(10, 21), Point2D::new(11, 21), Point2D::new(12, 21)
///     ],
///     Rect::new(Point2D::new(10, 20), Size2D::new(3, 2)).points().collect::<Vec<_>>());
/// ```
impl<T: Point2DRangeIteratorPrimitive, U> Points for euclid::TypedRect<T, U> {
    type Iter = Point2DRangeIterator<T, U>;

    fn points(self) -> Self::Iter {
        Point2DRangeIterator::new(self.origin, self.origin + self.size)
    }
}

/// # Examples
/// ```
/// # use euclid::{Box2D, Point2D};
/// # use euclid_ext::Points;
/// assert_eq!(
///     vec![
///         Point2D::new(10, 20), Point2D::new(11, 20), Point2D::new(12, 20),
///         Point2D::new(10, 21), Point2D::new(11, 21), Point2D::new(12, 21)
///     ],
///     Box2D::new(Point2D::new(10, 20), Point2D::new(13, 22)).points().collect::<Vec<_>>());
/// ```
impl<T: Point2DRangeIteratorPrimitive, U> Points for euclid::TypedBox2D<T, U> {
    type Iter = Point2DRangeIterator<T, U>;

    fn points(self) -> Self::Iter {
        Point2DRangeIterator::new(self.min, self.max)
    }
}

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::Points;
/// assert_eq!(
///     vec![
///         Point2D::new(10, 20), Point2D::new(11, 20), Point2D::new(12, 20),
///         Point2D::new(10, 21), Point2D::new(11, 21), Point2D::new(12, 21)
///     ],
///     (Point2D::new(10, 20)..Point2D::new(13, 22)).points().collect::<Vec<_>>());
/// ```
impl<T: Point2DRangeIteratorPrimitive, U> Points for ops::Range<euclid::TypedPoint2D<T, U>> {
    type Iter = Point2DRangeIterator<T, U>;

    fn points(self) -> Self::Iter {
        Point2DRangeIterator::new(self.start, self.end)
    }
}