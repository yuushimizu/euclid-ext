use euclid;
use std::ops;

pub type Point2D<T, U> = euclid::TypedPoint2D<T, U>;

pub type Point3D<T, U> = euclid::TypedPoint3D<T, U>;

pub type Point2DRange<T, U> = ops::Range<Point2D<T, U>>;

pub type Point3DRange<T, U> = ops::Range<Point3D<T, U>>;