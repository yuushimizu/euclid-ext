use std::marker;

/// # Examples
/// ```
/// # use euclid::Point2D;
/// const POINT: Point2D<i32> = euclid_ext::new::point2d(5, 8);
/// assert_eq!(Point2D::new(5, 8), POINT);
/// ```
pub const fn point2d<T, U>(x: T, y: T) -> euclid::TypedPoint2D<T, U> {
    euclid::TypedPoint2D {
        x,
        y,
        _unit: marker::PhantomData,
    }
}

/// # Examples
/// ```
/// # use euclid::Size2D;
/// const SIZE: Size2D<i32> = euclid_ext::new::size2d(11, 22);
/// assert_eq!(Size2D::new(11, 22), SIZE);
/// ```
pub const fn size2d<T, U>(width: T, height: T) -> euclid::TypedSize2D<T, U> {
    euclid::TypedSize2D {
        width,
        height,
        _unit: marker::PhantomData,
    }
}

/// # Examples
/// ```
/// # use euclid::Vector2D;
/// const VECTOR: Vector2D<i32> = euclid_ext::new::vector2d(9, 10);
/// assert_eq!(Vector2D::new(9, 10), VECTOR);
/// ```
pub const fn vector2d<T, U>(x: T, y: T) -> euclid::TypedVector2D<T, U> {
    euclid::TypedVector2D {
        x,
        y,
        _unit: marker::PhantomData,
    }
}

/// # Examples
/// ```
/// # use euclid::{Box2D, Point2D};
/// const BOX: Box2D<i32> = euclid_ext::new::box2d(euclid_ext::new::point2d(10, 20), euclid_ext::new::point2d(15, 26));
/// assert_eq!(Box2D::new(Point2D::new(10, 20), Point2D::new(15, 26)), BOX);
/// ```
pub const fn box2d<T, U>(
    min: euclid::TypedPoint2D<T, U>,
    max: euclid::TypedPoint2D<T, U>,
) -> euclid::TypedBox2D<T, U> {
    euclid::TypedBox2D { min, max }
}

/// # Examples
/// ```
/// # use euclid::{Rect, Point2D, Size2D};
/// const RECT: Rect<i32> = euclid_ext::new::rect(euclid_ext::new::point2d(5, 8), euclid_ext::new::size2d(10, 15));
/// assert_eq!(Rect::new(Point2D::new(5, 8), Size2D::new(10, 15)), RECT);
/// ```
pub const fn rect<T, U>(
    origin: euclid::TypedPoint2D<T, U>,
    size: euclid::TypedSize2D<T, U>,
) -> euclid::TypedRect<T, U> {
    euclid::TypedRect { origin, size }
}

/// # Examples
/// ```
/// # use euclid::Point3D;
/// const POINT: Point3D<i32> = euclid_ext::new::point3d(18, 3, 6);
/// assert_eq!(Point3D::new(18, 3, 6), POINT);
/// ```
pub const fn point3d<T, U>(x: T, y: T, z: T) -> euclid::TypedPoint3D<T, U> {
    euclid::TypedPoint3D {
        x,
        y,
        z,
        _unit: marker::PhantomData,
    }
}

/// # Examples
/// ```
/// # use euclid::Vector3D;
/// const VECTOR: Vector3D<i32> = euclid_ext::new::vector3d(7, 13, 19);
/// assert_eq!(Vector3D::new(7, 13, 19), VECTOR);
/// ```
pub const fn vector3d<T, U>(x: T, y: T, z: T) -> euclid::TypedVector3D<T, U> {
    euclid::TypedVector3D {
        x,
        y,
        z,
        _unit: marker::PhantomData,
    }
}

/// # Examples
/// ```
/// # use euclid::{Box3D, Point3D};
/// const BOX: Box3D<i32> = euclid_ext::new::box3d(euclid_ext::new::point3d(1, 2, 3), euclid_ext::new::point3d(5, 6, 7));
/// assert_eq!(Box3D::new(Point3D::new(1, 2, 3), Point3D::new(5, 6, 7)), BOX);
/// ```
pub const fn box3d<T, U>(
    min: euclid::TypedPoint3D<T, U>,
    max: euclid::TypedPoint3D<T, U>,
) -> euclid::TypedBox3D<T, U> {
    euclid::TypedBox3D { min, max }
}
