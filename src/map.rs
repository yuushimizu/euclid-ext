use crate::structure::{Structure2D, Structure3D};

pub trait Map2D {
    type Item;

    fn map<RI: Copy, R: Structure2D<Item = RI>>(self, f: impl FnMut(Self::Item) -> RI) -> R;
}

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::Map2D;
/// enum Space {}
/// assert_eq!(
///     Point2D::new(4, 6),
///     Point2D::<i64, Space>::new(2, 3).map(|n| n * 2));
/// ```
impl<T: Structure2D> Map2D for T {
    type Item = <Self as Structure2D>::Item;

    fn map<RI: Copy, R: Structure2D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y(f(self.x()), f(self.y()))
    }
}

/// # Examples
/// ```
/// # use euclid::Point2D;
/// # use euclid_ext::Map2D;
/// enum Space {}
/// assert_eq!(
///     Point2D::new(4, 6),
///     (Point2D::<i64, Space>::new(2, 3),).map(|(n,)| n * 2));
/// ```
impl<T: Structure2D> Map2D for (T,) {
    type Item = (<T as Structure2D>::Item,);

    fn map<RI: Copy, R: Structure2D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y(f((self.0.x(),)), f((self.0.y(),)))
    }
}

/// # Examples
/// ```
/// # use euclid::{Point2D, Vector2D};
/// # use euclid_ext::Map2D;
/// enum Space {}
/// assert_eq!(
///     Point2D::new(5, 12),
///     (Point2D::<i64, Space>::new(5, 8), Vector2D::<i64, Space>::new(2, 12)).map(|(n, m)| n.max(m)));
/// ```
impl<T0: Structure2D, T1: Structure2D> Map2D for (T0, T1) {
    type Item = (<T0 as Structure2D>::Item, <T1 as Structure2D>::Item);

    fn map<RI: Copy, R: Structure2D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y(f((self.0.x(), self.1.x())), f((self.0.y(), self.1.y())))
    }
}

/// # Examples
/// ```
/// # use euclid::{Point2D, Vector2D};
/// # use euclid_ext::Map2D;
/// enum Space {}
/// assert_eq!(
///     Vector2D::new(111, 222),
///     (Point2D::<i64, Space>::new(1, 2), Point2D::<i64, Space>::new(10, 20), Point2D::<i64, Space>::new(100, 200)).map(|(a, b, c)| a + b + c));
/// ```
impl<T0: Structure2D, T1: Structure2D, T2: Structure2D> Map2D for (T0, T1, T2) {
    type Item = (
        <T0 as Structure2D>::Item,
        <T1 as Structure2D>::Item,
        <T2 as Structure2D>::Item,
    );

    fn map<RI: Copy, R: Structure2D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y(
            f((self.0.x(), self.1.x(), self.2.x())),
            f((self.0.y(), self.1.y(), self.2.y())),
        )
    }
}

pub trait Map3D {
    type Item;

    fn map<RI: Copy, R: Structure3D<Item = RI>>(self, f: impl FnMut(Self::Item) -> RI) -> R;
}

/// # Examples
/// ```
/// # use euclid::Point3D;
/// # use euclid_ext::Map3D;
/// enum Space {}
/// assert_eq!(
///     Point3D::new(4, 6, 10),
///     Point3D::<i64, Space>::new(2, 3, 5).map(|n| n * 2));
/// ```
impl<T: Structure3D> Map3D for T {
    type Item = <Self as Structure3D>::Item;

    fn map<RI: Copy, R: Structure3D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y_z(f(self.x()), f(self.y()), f(self.z()))
    }
}

/// # Examples
/// ```
/// # use euclid::Point3D;
/// # use euclid_ext::Map3D;
/// enum Space {}
/// assert_eq!(
///     Point3D::new(4, 6, 10),
///     (Point3D::<i64, Space>::new(2, 3, 5),).map(|(n,)| n * 2));
/// ```
impl<T: Structure3D> Map3D for (T,) {
    type Item = (<T as Structure3D>::Item,);

    fn map<RI: Copy, R: Structure3D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y_z(f((self.0.x(),)), f((self.0.y(),)), f((self.0.z(),)))
    }
}

/// # Examples
/// ```
/// # use euclid::Point3D;
/// # use euclid::Vector3D;
/// # use euclid_ext::Map3D;
/// enum Space {}
/// assert_eq!(
///     Point3D::new(5, 12, 7),
///     (Point3D::<i64, Space>::new(5, 8, 2), Vector3D::<i64, Space>::new(2, 12, 7)).map(|(n, m)| n.max(m)));
/// ```
impl<T0: Structure3D, T1: Structure3D> Map3D for (T0, T1) {
    type Item = (<T0 as Structure3D>::Item, <T1 as Structure3D>::Item);

    fn map<RI: Copy, R: Structure3D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y_z(
            f((self.0.x(), self.1.x())),
            f((self.0.y(), self.1.y())),
            f((self.0.z(), self.1.z())),
        )
    }
}

/// # Examples
/// ```
/// # use euclid::{Point3D, Vector3D};
/// # use euclid_ext::Map3D;
/// enum Space {}
/// assert_eq!(
///     Vector3D::new(111, 222, 333),
///     (Point3D::<i64, Space>::new(1, 2, 3), Point3D::<i64, Space>::new(10, 20, 30), Point3D::<i64, Space>::new(100, 200, 300)).map(|(a, b, c)| a + b + c));
/// ```
impl<T0: Structure3D, T1: Structure3D, T2: Structure3D> Map3D for (T0, T1, T2) {
    type Item = (
        <T0 as Structure3D>::Item,
        <T1 as Structure3D>::Item,
        <T2 as Structure3D>::Item,
    );

    fn map<RI: Copy, R: Structure3D<Item = RI>>(self, mut f: impl FnMut(Self::Item) -> RI) -> R {
        R::from_x_y_z(
            f((self.0.x(), self.1.x(), self.2.x())),
            f((self.0.y(), self.1.y(), self.2.y())),
            f((self.0.z(), self.1.z(), self.2.z())),
        )
    }
}
