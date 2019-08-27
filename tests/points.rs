use euclid::{Box2D, Box3D, Rect, Size2D};
use euclid_ext::Points;

enum Space {}

type Point2D<T> = euclid::Point2D<T, Space>;

type Point3D<T> = euclid::Point3D<T, Space>;

macro_rules! point_vec {
    ($(($x:expr, $y:expr, $z:expr)),*) => {
        vec![$(Point3D::new($x, $y, $z)),*]
    };
    ($(($x:expr, $y:expr)),*) => {
        vec![$(Point2D::new($x, $y)),*]
    };
}

#[test]
fn points_in_range_2d() {
    assert_eq!(
        point_vec![(8, 3), (9, 3), (10, 3), (8, 4), (9, 4), (10, 4)],
        (Point2D::new(8, 3)..Point2D::new(11, 5))
            .points()
            .collect::<Vec<_>>()
    );
}

#[test]
fn points_in_box2d() {
    assert_eq!(
        point_vec![(5, 10), (6, 10), (7, 10), (5, 11), (6, 11), (7, 11)],
        Box2D::new(Point2D::new(5, 10), Point2D::new(8, 12))
            .points()
            .collect::<Vec<_>>()
    );
}

#[test]
fn points_in_rect() {
    assert_eq!(
        point_vec![
            (10, 13),
            (11, 13),
            (12, 13),
            (13, 13),
            (10, 14),
            (11, 14),
            (12, 14),
            (13, 14),
            (10, 15),
            (11, 15),
            (12, 15),
            (13, 15)
        ],
        Rect::new(Point2D::new(10, 13), Size2D::new(4, 3))
            .points()
            .collect::<Vec<_>>()
    );
}

#[test]
fn points_in_range_3d() {
    assert_eq!(
        point_vec![
            (3, 5, 7),
            (4, 5, 7),
            (3, 6, 7),
            (4, 6, 7),
            (3, 7, 7),
            (4, 7, 7),
            (3, 5, 8),
            (4, 5, 8),
            (3, 6, 8),
            (4, 6, 8),
            (3, 7, 8),
            (4, 7, 8),
            (3, 5, 9),
            (4, 5, 9),
            (3, 6, 9),
            (4, 6, 9),
            (3, 7, 9),
            (4, 7, 9),
            (3, 5, 10),
            (4, 5, 10),
            (3, 6, 10),
            (4, 6, 10),
            (3, 7, 10),
            (4, 7, 10)
        ],
        (Point3D::new(3, 5, 7)..Point3D::new(5, 8, 11))
            .points()
            .collect::<Vec<_>>()
    );
}

#[test]
fn points_in_box3d() {
    assert_eq!(
        point_vec![
            (0, 11, 22),
            (1, 11, 22),
            (2, 11, 22),
            (3, 11, 22),
            (0, 12, 22),
            (1, 12, 22),
            (2, 12, 22),
            (3, 12, 22),
            (0, 13, 22),
            (1, 13, 22),
            (2, 13, 22),
            (3, 13, 22),
            (0, 11, 23),
            (1, 11, 23),
            (2, 11, 23),
            (3, 11, 23),
            (0, 12, 23),
            (1, 12, 23),
            (2, 12, 23),
            (3, 12, 23),
            (0, 13, 23),
            (1, 13, 23),
            (2, 13, 23),
            (3, 13, 23)
        ],
        Box3D::new(Point3D::new(0, 11, 22), Point3D::new(4, 14, 24))
            .points()
            .collect::<Vec<_>>()
    );
}
