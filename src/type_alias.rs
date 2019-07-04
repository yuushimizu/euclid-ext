use euclid;
use std::ops;

pub type Vector2D<T, U> = euclid::TypedVector2D<T, U>;

pub type Point2D<T, U> = euclid::TypedPoint2D<T, U>;

pub type Point2DRange<T, U> = ops::Range<Point2D<T, U>>;

pub type Size2D<T, U> = euclid::TypedSize2D<T, U>;

pub type Box2D<T, U> = euclid::TypedBox2D<T, U>;

pub type Rect<T, U> = euclid::TypedRect<T, U>;

pub type Vector3D<T, U> = euclid::TypedVector3D<T, U>;

pub type Point3D<T, U> = euclid::TypedPoint3D<T, U>;

pub type Point3DRange<T, U> = ops::Range<Point3D<T, U>>;

pub type Box3D<T, U> = euclid::TypedBox3D<T, U>;
