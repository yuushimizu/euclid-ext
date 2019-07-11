enum Space {}

#[test]
fn point2d() {
    const POINT: euclid::TypedPoint2D<i32, Space> = euclid_ext::new::point2d(10, 20);
    assert_eq!(euclid::TypedPoint2D::new(10, 20), POINT);
}

#[test]
fn size2d() {
    const SIZE: euclid::TypedSize2D<usize, Space> = euclid_ext::new::size2d(5, 10);
    assert_eq!(euclid::TypedSize2D::new(5, 10), SIZE);
}

#[test]
fn vector2d() {
    const VECTOR: euclid::TypedVector2D<f64, Space> = euclid_ext::new::vector2d(8.3, 12.5);
    assert_eq!(euclid::TypedVector2D::new(8.3, 12.5), VECTOR);
}

#[test]
fn rect() {
    const RECT: euclid::TypedRect<usize, Space> =
        euclid_ext::new::rect(euclid_ext::new::point2d(50, 100), euclid_ext::new::size2d(200, 200));
    assert_eq!(
        euclid::TypedRect::new(
            euclid::TypedPoint2D::new(50, 100),
            euclid::TypedSize2D::new(200, 200)
        ),
        RECT
    );
}

#[test]
fn point3d() {
    const POINT: euclid::TypedPoint3D<u8, Space> = euclid_ext::new::point3d(3, 7, 5);
    assert_eq!(euclid::TypedPoint3D::new(3, 7, 5), POINT);
}

#[test]
fn vector3d() {
    const VECTOR: euclid::TypedVector3D<i32, Space> = euclid_ext::new::vector3d(10, 12, 14);
    assert_eq!(euclid::TypedVector3D::new(10, 12, 14), VECTOR);
}

#[test]
fn box3d() {
    const BOX: euclid::TypedBox3D<f32, Space> = euclid_ext::new::box3d(
        euclid_ext::new::point3d(1.4, 2.9, 3.1),
        euclid_ext::new::point3d(4.6, 5.7, 6.2),
    );
    assert_eq!(
        euclid::TypedBox3D::new(
            euclid::TypedPoint3D::new(1.4, 2.9, 3.1),
            euclid::TypedPoint3D::new(4.6, 5.7, 6.2)
        ),
        BOX
    );
}
