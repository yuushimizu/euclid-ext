use crate::type_alias::{Box2D, Box3D, Point2D, Point3D, Rect, Size2D, Vector2D, Vector3D};
use euclid::Length;

pub trait Structure2D {
    type Item;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self;

    fn x(&self) -> Self::Item;

    fn y(&self) -> Self::Item;
}

impl<T: Copy, U> Structure2D for Point2D<T, U> {
    type Item = Length<T, U>;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::from_lengths(x, y)
    }

    fn x(&self) -> Self::Item {
        self.x_typed()
    }

    fn y(&self) -> Self::Item {
        self.y_typed()
    }
}

impl<T: Copy, U> Structure2D for Size2D<T, U> {
    type Item = Length<T, U>;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::from_lengths(x, y)
    }

    fn x(&self) -> Self::Item {
        self.width_typed()
    }

    fn y(&self) -> Self::Item {
        self.height_typed()
    }
}

impl<T: Copy, U> Structure2D for Vector2D<T, U> {
    type Item = Length<T, U>;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::from_lengths(x, y)
    }

    fn x(&self) -> Self::Item {
        self.x_typed()
    }

    fn y(&self) -> Self::Item {
        self.y_typed()
    }
}

impl<T: Copy, U> Structure2D for Box2D<T, U> {
    type Item = (Length<T, U>, Length<T, U>);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::new(
            Point2D::from_lengths(x.0, y.0),
            Point2D::from_lengths(x.1, y.1),
        )
    }

    fn x(&self) -> Self::Item {
        (self.min.x_typed(), self.max.x_typed())
    }

    fn y(&self) -> Self::Item {
        (self.min.y_typed(), self.max.y_typed())
    }
}

impl<T: Copy, U> Structure2D for Rect<T, U> {
    type Item = (Length<T, U>, Length<T, U>);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::new(
            Point2D::from_lengths(x.0, y.0),
            Size2D::from_lengths(x.1, y.1),
        )
    }

    fn x(&self) -> Self::Item {
        (self.origin.x_typed(), self.size.width_typed())
    }

    fn y(&self) -> Self::Item {
        (self.origin.y_typed(), self.size.height_typed())
    }
}

pub trait Structure3D {
    type Item;

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self;

    fn x(&self) -> Self::Item;

    fn y(&self) -> Self::Item;

    fn z(&self) -> Self::Item;
}

impl<T: Copy, U> Structure3D for Point3D<T, U> {
    type Item = Length<T, U>;

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self {
        Self::from_lengths(x, y, z)
    }

    fn x(&self) -> Self::Item {
        self.x_typed()
    }

    fn y(&self) -> Self::Item {
        self.y_typed()
    }

    fn z(&self) -> Self::Item {
        self.z_typed()
    }
}

impl<T: Copy, U> Structure3D for Vector3D<T, U> {
    type Item = Length<T, U>;

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self {
        Self::from_lengths(x, y, z)
    }

    fn x(&self) -> Self::Item {
        self.x_typed()
    }

    fn y(&self) -> Self::Item {
        self.y_typed()
    }

    fn z(&self) -> Self::Item {
        self.z_typed()
    }
}

impl<T: Copy, U> Structure3D for Box3D<T, U> {
    type Item = (Length<T, U>, Length<T, U>);

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self {
        Self::new(
            Point3D::from_lengths(x.0, y.0, z.0),
            Point3D::from_lengths(x.1, y.1, z.1),
        )
    }

    fn x(&self) -> Self::Item {
        (self.min.x_typed(), self.max.x_typed())
    }

    fn y(&self) -> Self::Item {
        (self.min.y_typed(), self.max.y_typed())
    }

    fn z(&self) -> Self::Item {
        (self.min.z_typed(), self.max.z_typed())
    }
}