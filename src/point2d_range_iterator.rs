use euclid;
use std::ops;

type Point<T, U> = euclid::TypedPoint2D<T, U>;

pub trait Point2DRangeIteratorPrimitive:
    Copy + PartialOrd + ops::Add<Output = Self> + euclid::num::One
{
}

impl<T: Copy + PartialOrd + ops::Add<Output = Self> + euclid::num::One>
    Point2DRangeIteratorPrimitive for T
{
}

pub struct Point2DRangeIterator<T: Point2DRangeIteratorPrimitive, U> {
    start: Point<T, U>,
    end: Point<T, U>,
    current: Point<T, U>,
}

impl<T: Point2DRangeIteratorPrimitive, U> Point2DRangeIterator<T, U> {
    pub fn new(start: Point<T, U>, end: Point<T, U>) -> Self {
        Self {
            start,
            end,
            current: start,
        }
    }
}

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::Point2DRangeIterator;
/// let mut i = Point2DRangeIterator::new(Point2D::new(10, 20), Point2D::new(12, 22));
/// assert_eq!(Some(Point2D::new(10, 20)), i.next());
/// assert_eq!(Some(Point2D::new(11, 20)), i.next());
/// assert_eq!(Some(Point2D::new(10, 21)), i.next());
/// assert_eq!(Some(Point2D::new(11, 21)), i.next());
/// assert_eq!(None, i.next());
/// ```
impl<T: Point2DRangeIteratorPrimitive, U> Iterator for Point2DRangeIterator<T, U> {
    type Item = Point<T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current.y < self.end.y {
            if self.current.x < self.end.x {
                let result = Some(self.current);
                self.current.x = self.current.x + T::one();
                return result;
            }
            self.current = Point::new(self.start.x, self.current.y + T::one());
        }
        None
    }
}