use crate::type_alias::{Box2D, Box3D, Point2D, Point2DRange, Point3D, Point3DRange, Rect, Size2D};
use std::ops;

pub trait ToPointRange {
    type Point;

    fn to_point_range(self) -> ops::Range<Self::Point>;
}

impl<T, U> ToPointRange for Point2DRange<T, U> {
    type Point = Point2D<T, U>;

    fn to_point_range(self) -> Self {
        self
    }
}

impl<T, U> ToPointRange for Rect<T, U>
where
    Point2D<T, U>: Clone + ops::Add<Size2D<T, U>, Output = Point2D<T, U>>,
{
    type Point = Point2D<T, U>;

    fn to_point_range(self) -> Point2DRange<T, U> {
        let end = self.origin.clone() + self.size;
        self.origin..end
    }
}

impl<T, U> ToPointRange for Box2D<T, U> {
    type Point = Point2D<T, U>;

    fn to_point_range(self) -> Point2DRange<T, U> {
        self.min..self.max
    }
}

impl<T, U> ToPointRange for Point3DRange<T, U> {
    type Point = Point3D<T, U>;

    fn to_point_range(self) -> Self {
        self
    }
}

impl<T, U> ToPointRange for Box3D<T, U> {
    type Point = Point3D<T, U>;

    fn to_point_range(self) -> Point3DRange<T, U> {
        self.min..self.max
    }
}