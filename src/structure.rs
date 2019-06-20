use euclid::Length;

pub trait Structure2D {
    type Item;

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self;

    fn x(&self) -> Self::Item;

    fn y(&self) -> Self::Item;
}

impl<T: Copy, U> Structure2D for euclid::TypedPoint2D<T, U> {
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

impl<T: Copy, U> Structure2D for euclid::TypedSize2D<T, U> {
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

impl<T: Copy, U> Structure2D for euclid::TypedVector2D<T, U> {
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

impl<T: Copy, U> Structure2D for euclid::TypedBox2D<T, U> {
    type Item = (Length<T, U>, Length<T, U>);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::new(
            euclid::TypedPoint2D::from_lengths(x.0, y.0),
            euclid::TypedPoint2D::from_lengths(x.1, y.1),
        )
    }

    fn x(&self) -> Self::Item {
        (self.min.x_typed(), self.max.x_typed())
    }

    fn y(&self) -> Self::Item {
        (self.min.y_typed(), self.max.y_typed())
    }
}

impl<T: Copy, U> Structure2D for euclid::TypedRect<T, U> {
    type Item = (Length<T, U>, Length<T, U>);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        Self::new(
            euclid::TypedPoint2D::from_lengths(x.0, y.0),
            euclid::TypedSize2D::from_lengths(x.1, y.1),
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

impl<T: Copy, U> Structure3D for euclid::TypedPoint3D<T, U> {
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

impl<T: Copy, U> Structure3D for euclid::TypedVector3D<T, U> {
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

impl<T: Copy, U> Structure3D for euclid::TypedBox3D<T, U> {
    type Item = (Length<T, U>, Length<T, U>);

    fn from_x_y_z(x: Self::Item, y: Self::Item, z: Self::Item) -> Self {
        Self::new(
            euclid::TypedPoint3D::from_lengths(x.0, y.0, z.0),
            euclid::TypedPoint3D::from_lengths(x.1, y.1, z.1),
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