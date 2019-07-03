use euclid;
use std::ops;

type Point2D<T, U> = euclid::TypedPoint2D<T, U>;

type Point3D<T, U> = euclid::TypedPoint3D<T, U>;

type Point2DRange<T, U> = ops::Range<Point2D<T, U>>;

type Point3DRange<T, U> = ops::Range<Point3D<T, U>>;

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

impl<T, U> ToPointRange for euclid::TypedRect<T, U>
where
    T: Copy + ops::Add<Output = T>,
{
    type Point = Point2D<T, U>;
    
    fn to_point_range(self) -> Point2DRange<T, U> {
        self.origin..self.origin + self.size
    }
}

impl<T, U> ToPointRange for euclid::TypedBox2D<T, U> {
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

impl<T, U> ToPointRange for euclid::TypedBox3D<T, U> {
    type Point = Point3D<T, U>;
    
    fn to_point_range(self) -> Point3DRange<T, U> {
        self.min..self.max
    }
}