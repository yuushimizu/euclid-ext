pub trait Structure2D {
    type Item;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self;

    fn x(&self) -> Self::Item;

    fn y(&self) -> Self::Item;
}

impl<T: Copy, U> Structure2D for euclid::Point2D<T, U> {
    type Item = euclid::Length<T, U>;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::from_lengths(x, y)
    }

    fn x(&self) -> Self::Item {
        Self::Item::new(self.x)
    }

    fn y(&self) -> Self::Item {
        Self::Item::new(self.y)
    }
}

impl<T: Copy, U> Structure2D for euclid::Size2D<T, U> {
    type Item = euclid::Length<T, U>;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::from_lengths(x, y)
    }

    fn x(&self) -> Self::Item {
        Self::Item::new(self.width)
    }

    fn y(&self) -> Self::Item {
        Self::Item::new(self.height)
    }
}

impl<T: Copy, U> Structure2D for euclid::Vector2D<T, U> {
    type Item = euclid::Length<T, U>;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::from_lengths(x, y)
    }

    fn x(&self) -> Self::Item {
        Self::Item::new(self.x)
    }

    fn y(&self) -> Self::Item {
        Self::Item::new(self.y)
    }
}

impl<T: Copy, U> Structure2D for euclid::Box2D<T, U> {
    type Item = (euclid::Length<T, U>, euclid::Length<T, U>);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::new(
            euclid::Point2D::from_lengths(x.0, y.0),
            euclid::Point2D::from_lengths(x.1, y.1),
        )
    }

    fn x(&self) -> Self::Item {
        (self.min.x(), self.max.x())
    }

    fn y(&self) -> Self::Item {
        (self.min.y(), self.max.y())
    }
}

impl<T: Copy, U> Structure2D for euclid::Rect<T, U> {
    type Item = (euclid::Length<T, U>, euclid::Length<T, U>);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::new(
            euclid::Point2D::from_lengths(x.0, y.0),
            euclid::Size2D::from_lengths(x.1, y.1),
        )
    }

    fn x(&self) -> Self::Item {
        (self.origin.x(), self.size.x())
    }

    fn y(&self) -> Self::Item {
        (self.origin.y(), self.size.y())
    }
}

pub trait Structure3D {
    type Item;

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self;

    fn x(&self) -> Self::Item;

    fn y(&self) -> Self::Item;

    fn z(&self) -> Self::Item;
}

impl<T: Copy, U> Structure3D for euclid::Point3D<T, U> {
    type Item = euclid::Length<T, U>;

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self {
        Self::from_lengths(x, y, z)
    }

    fn x(&self) -> Self::Item {
        Self::Item::new(self.x)
    }

    fn y(&self) -> Self::Item {
        Self::Item::new(self.y)
    }

    fn z(&self) -> Self::Item {
        Self::Item::new(self.z)
    }
}

impl<T: Copy, U> Structure3D for euclid::Vector3D<T, U> {
    type Item = euclid::Length<T, U>;

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self {
        Self::from_lengths(x, y, z)
    }

    fn x(&self) -> Self::Item {
        Self::Item::new(self.x)
    }

    fn y(&self) -> Self::Item {
        Self::Item::new(self.y)
    }

    fn z(&self) -> Self::Item {
        Self::Item::new(self.z)
    }
}

impl<T: Copy, U> Structure3D for euclid::Box3D<T, U> {
    type Item = (euclid::Length<T, U>, euclid::Length<T, U>);

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self {
        Self::new(
            euclid::Point3D::from_lengths(x.0, y.0, z.0),
            euclid::Point3D::from_lengths(x.1, y.1, z.1),
        )
    }

    fn x(&self) -> Self::Item {
        (self.min.x(), self.max.x())
    }

    fn y(&self) -> Self::Item {
        (self.min.y(), self.max.y())
    }

    fn z(&self) -> Self::Item {
        (self.min.z(), self.max.z())
    }
}