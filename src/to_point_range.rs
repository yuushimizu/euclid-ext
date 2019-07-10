use std::ops;

pub trait ToPointRange {
    type Point;

    fn to_point_range(self) -> ops::Range<Self::Point>;
}

impl<T, U> ToPointRange for ops::Range<euclid::TypedPoint2D<T, U>> {
    type Point = euclid::TypedPoint2D<T, U>;

    fn to_point_range(self) -> Self {
        self
    }
}

impl<T, U> ToPointRange for euclid::TypedRect<T, U>
where
    euclid::TypedPoint2D<T, U>:
        Clone + ops::Add<euclid::TypedSize2D<T, U>, Output = euclid::TypedPoint2D<T, U>>,
{
    type Point = euclid::TypedPoint2D<T, U>;

    fn to_point_range(self) -> ops::Range<Self::Point> {
        let end = self.origin.clone() + self.size;
        self.origin..end
    }
}

impl<T, U> ToPointRange for euclid::TypedBox2D<T, U> {
    type Point = euclid::TypedPoint2D<T, U>;

    fn to_point_range(self) -> ops::Range<Self::Point> {
        self.min..self.max
    }
}

impl<T, U> ToPointRange for ops::Range<euclid::TypedPoint3D<T, U>> {
    type Point = euclid::TypedPoint3D<T, U>;

    fn to_point_range(self) -> Self {
        self
    }
}

impl<T, U> ToPointRange for euclid::TypedBox3D<T, U> {
    type Point = euclid::TypedPoint3D<T, U>;

    fn to_point_range(self) -> ops::Range<Self::Point> {
        self.min..self.max
    }
}