use euclid::{
    Length, TypedBox2D, TypedBox3D, TypedPoint2D, TypedPoint3D, TypedRect, TypedSize2D,
    TypedVector2D, TypedVector3D,
};
use euclid_ext::{Map2D, Map3D};

enum Space {}

type Point2D<T> = TypedPoint2D<T, Space>;

type Size2D<T> = TypedSize2D<T, Space>;

type Vector2D<T> = TypedVector2D<T, Space>;

type Box2D<T> = TypedBox2D<T, Space>;

type Rect<T> = TypedRect<T, Space>;

type Point3D<T> = TypedPoint3D<T, Space>;

type Vector3D<T> = TypedVector3D<T, Space>;

type Box3D<T> = TypedBox3D<T, Space>;

#[test]
fn map_point2d() {
    assert_eq!(Point2D::new(14, 16), Point2D::new(7, 8).map(|n| n * 2));
}

#[test]
fn map_size2d() {
    assert_eq!(
        Size2D::new(13, 17),
        Size2D::new(3, 7).map(|n| n + Length::new(10))
    );
}

#[test]
fn map_vector2d() {
    assert_eq!(Vector2D::new(60, 50), Vector2D::new(6, 5).map(|n| n * 10));
}

#[test]
fn map_box2d() {
    assert_eq!(
        Box2D::new(Point2D::new(4, 6), Point2D::new(70, 80)),
        Box2D::new(Point2D::new(2, 3), Point2D::new(7, 8)).map(|(min, max)| (min * 2, max * 10))
    );
}

#[test]
fn map_rect() {
    assert_eq!(
        Rect::new(Point2D::new(15, 18), Size2D::new(500, 700)),
        Rect::new(Point2D::new(5, 8), Size2D::new(5, 7))
            .map(|(origin, size)| (origin + Length::new(10), size * 100))
    )
}

#[test]
fn map_to_another_type_2d() {
    assert_eq!(Vector2D::new(10, 20), Point2D::new(5, 10).map(|n| n * 2));
}

#[test]
fn map_to_box2d() {
    assert_eq!(
        Box2D::new(Point2D::new(2, 4), Point2D::new(10, 20)),
        Point2D::new(1, 2).map(|n| (n * 2, n * 10))
    );
}

#[test]
fn map_to_rect() {
    assert_eq!(
        Rect::new(Point2D::new(5, 4), Size2D::new(15, 14)),
        Point2D::new(5, 4).map(|n| (n, n + Length::new(10)))
    );
}

#[test]
fn map_single_tuple_2d() {
    assert_eq!(Point2D::new(9, 15), (Point2D::new(3, 5),).map(|(n,)| n * 3));
}

#[test]
fn map_pair_2d() {
    assert_eq!(
        Point2D::new(18, 22),
        (Point2D::new(11, 8), Vector2D::new(7, 14)).map(|(n, m)| n + m)
    );
}

#[test]
fn map_triple_2d() {
    assert_eq!(
        Vector2D::new(184, 275),
        (Size2D::new(1, 2), Point2D::new(8, 7), Vector2D::new(4, 5))
            .map(|(a, b, c)| a * 100 + b * 10 + c)
    );
}

#[test]
fn map_point3d() {
    assert_eq!(Point3D::new(2, 4, 6), Point3D::new(1, 2, 3).map(|n| n * 2));
}

#[test]
fn map_vector3d() {
    assert_eq!(
        Vector3D::new(11, 12, 13),
        Vector3D::new(1, 2, 3).map(|n| n + Length::new(10))
    );
}

#[test]
fn map_box3d() {
    assert_eq!(
        Box3D::new(Point3D::new(8, 12, 4), Point3D::new(25, 28, 29)),
        Box3D::new(Point3D::new(4, 6, 2), Point3D::new(5, 8, 9))
            .map(|(min, max)| (min * 2, max + Length::new(20)))
    );
}

#[test]
fn map_to_another_type_3d() {
    assert_eq!(
        Vector3D::new(5, 10, 15),
        Point3D::new(1, 2, 3).map(|n| n * 5)
    );
}

#[test]
fn map_to_box3d() {
    assert_eq!(
        Box3D::new(Point3D::new(2, 4, 6), Point3D::new(11, 12, 13)),
        Point3D::new(1, 2, 3).map(|n| (n * 2, n + Length::new(10)))
    );
}

#[test]
fn map_single_tuple_3d() {
    assert_eq!(
        Point3D::new(14, 42, 34),
        (Point3D::new(7, 21, 17),).map(|(n,)| n * 2)
    );
}

#[test]
fn map_pair_3d() {
    assert_eq!(
        Vector3D::new(24, 89, 44),
        (Point3D::new(20, 80, 40), Point3D::new(4, 9, 4)).map(|(n, m)| n + m)
    );
}

#[test]
fn map_triple_3d() {
    assert_eq!(
        Point3D::new(234, 854, 921),
        (
            Vector3D::new(2, 8, 9),
            Point3D::new(3, 5, 2),
            Vector3D::new(4, 4, 1)
        )
            .map(|(a, b, c)| a * 100 + b * 10 + c)
    );
}