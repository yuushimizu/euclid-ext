use std::ops;

pub trait ToPointRange {
    type Point;

    fn to_point_range(self) -> ops::Range<Self::Point>;
}

impl<T, U> ToPointRange for ops::Range<euclid::Point2D<T, U>> {
    type Point = euclid::Point2D<T, U>;

    fn to_point_range(self) -> Self {
        self
    }
}

impl<T, U> ToPointRange for euclid::Rect<T, U>
where
    euclid::Point2D<T, U>:
        Clone + ops::Add<euclid::Size2D<T, U>, Output = euclid::Point2D<T, U>>,
{
    type Point = euclid::Point2D<T, U>;

    fn to_point_range(self) -> ops::Range<Self::Point> {
        let end = self.origin.clone() + self.size;
        self.origin..end
    }
}

impl<T, U> ToPointRange for euclid::Box2D<T, U> {
    type Point = euclid::Point2D<T, U>;

    fn to_point_range(self) -> ops::Range<Self::Point> {
        self.min..self.max
    }
}

impl<T, U> ToPointRange for ops::Range<euclid::Point3D<T, U>> {
    type Point = euclid::Point3D<T, U>;

    fn to_point_range(self) -> Self {
        self
    }
}

impl<T, U> ToPointRange for euclid::Box3D<T, U> {
    type Point = euclid::Point3D<T, U>;

    fn to_point_range(self) -> ops::Range<Self::Point> {
        self.min..self.max
    }
}