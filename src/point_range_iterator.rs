use std::ops;

pub trait PointRangeIteratorPrimitive:
    Copy + PartialOrd + ops::Add<Output = Self> + euclid::num::One
{
}

impl<T: Copy + PartialOrd + ops::Add<Output = Self> + euclid::num::One> PointRangeIteratorPrimitive
    for T
{
}

pub trait PointRangeIteratorItem: Copy + Sized {
    fn next(&mut self, range: &ops::Range<Self>) -> Option<Self>;
}

impl<T: PointRangeIteratorPrimitive, U> PointRangeIteratorItem for euclid::TypedPoint2D<T, U> {
    fn next(&mut self, range: &ops::Range<Self>) -> Option<Self> {
        while self.y < range.end.y {
            if self.x < range.end.x {
                let result = Some(*self);
                self.x = self.x + T::one();
                return result;
            }
            *self = Self::new(range.start.x, self.y + T::one());
        }
        None
    }
}

impl<T: PointRangeIteratorPrimitive, U> PointRangeIteratorItem for euclid::TypedPoint3D<T, U> {
    fn next(&mut self, range: &ops::Range<Self>) -> Option<Self> {
        while self.z < range.end.z {
            while self.y < range.end.y {
                if self.x < range.end.x {
                    let result = Some(*self);
                    self.x = self.x + T::one();
                    return result;
                }
                *self = Self::new(range.start.x, self.y + T::one(), self.z);
            }
            *self = Self::new(range.start.x, range.start.y, self.z + T::one());
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointRangeIterator<T: PointRangeIteratorItem> {
    range: ops::Range<T>,
    current: T,
}

impl<T: PointRangeIteratorItem> PointRangeIterator<T> {
    pub fn new(range: ops::Range<T>) -> Self {
        let current = range.start;
        Self { range, current }
    }
}

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::point_range_iterator::PointRangeIterator;
/// let mut i = PointRangeIterator::new(Point2D::new(10, 20)..Point2D::new(12, 22));
/// assert_eq!(Some(Point2D::new(10, 20)), i.next());
/// assert_eq!(Some(Point2D::new(11, 20)), i.next());
/// assert_eq!(Some(Point2D::new(10, 21)), i.next());
/// assert_eq!(Some(Point2D::new(11, 21)), i.next());
/// assert_eq!(None, i.next());
/// ```
impl<T: PointRangeIteratorItem> Iterator for PointRangeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.next(&self.range)
    }
}