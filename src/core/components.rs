use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

/// Position component for entities in the game world
#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
#[reflect(Component)]
pub struct Position(pub IVec2);

impl Position {
    /// Creates a new `Position` component from the given x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new(3, 7);
    /// assert_eq!(pos.x(), 3);
    /// assert_eq!(pos.y(), 7);
    /// ```
    pub fn new(x: i32, y: i32) -> Self { 
        Self(IVec2::new(x, y)) 
    }

    /// Returns the x-coordinate of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new(3, 7);
    /// assert_eq!(pos.x(), 3);
    /// ```
    pub fn x(&self) -> i32 { 
        self.0.x 
    }

    /// Returns the y-coordinate of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new(3, 7);
    /// assert_eq!(pos.y(), 7);
    /// ```
    pub fn y(&self) -> i32 { 
        self.0.y 
    }
}

impl From<IVec2> for Position {
    /// Creates a `Position` from a given `IVec2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::IVec2;
    /// let vec = IVec2::new(3, 7);
    /// let pos = Position::from(vec);
    /// assert_eq!(pos.x(), 3);
    /// assert_eq!(pos.y(), 7);
    /// ```
    fn from(vec: IVec2) -> Self { 
        Self(vec) 
    }
}

impl From<Position> for (i32, i32) {
    /// Converts a `Position` into a tuple containing its x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new(3, 7);
    /// let coords: (i32, i32) = pos.into();
    /// assert_eq!(coords, (3, 7));
    /// ```
    fn from(value: Position) -> Self { 
        (value.0.x, value.0.y) 
    }
}

impl From<TilePos> for Position {
    /// Converts a `TilePos` from `bevy_ecs_tilemap` into a `Position` component.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_ecs_tilemap::tiles::TilePos;
    /// let tile_pos = TilePos { x: 3, y: 7 };
    /// let pos = Position::from(tile_pos);
    /// assert_eq!(pos.x(), 3);
    /// assert_eq!(pos.y(), 7);
    /// ```
    fn from(tile_pos: TilePos) -> Self { 
        Self(IVec2::new(tile_pos.x as i32, tile_pos.y as i32)) 
    }
}

impl Add<Position> for Position {
    type Output = Self;

    /// Returns the sum of two `Position` values as a new `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = Position::new(2, 3);
    /// let b = Position::new(4, 1);
    /// let c = a + b;
    /// assert_eq!(c, Position::new(6, 4));
    /// ```
    fn add(self, rhs: Position) -> Self::Output { 
        Self(self.0 + rhs.0) 
    }
}

impl AddAssign<Position> for Position {
    /// Adds the coordinates of another `Position` to this one in place.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut pos = Position::new(2, 3);
    /// pos += Position::new(1, 4);
    /// assert_eq!(pos, Position::new(3, 7));
    /// ```
    fn add_assign(&mut self, rhs: Position) { 
        self.0 += rhs.0; 
    }
}

impl Add<IVec2> for Position {
    type Output = Self;

    /// Returns a new `Position` by adding the given `IVec2` offset to this position.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::IVec2;
    /// let pos = Position::new(2, 3);
    /// let offset = IVec2::new(1, -1);
    /// let result = pos + offset;
    /// assert_eq!(result, Position::new(3, 2));
    /// ```
    fn add(self, rhs: IVec2) -> Self::Output { 
        Self(self.0 + rhs) 
    }
}

impl AddAssign<IVec2> for Position {
    /// Adds the given `IVec2` to this `Position` in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::math::IVec2;
    /// let mut pos = Position::new(1, 2);
    /// pos += IVec2::new(3, 4);
    /// assert_eq!(pos, Position::new(4, 6));
    /// ```
    fn add_assign(&mut self, rhs: IVec2) { 
        self.0 += rhs; 
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Self;

    /// Returns a new `Position` by adding the given `(i32, i32)` tuple to this position's coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = Position::new(2, 3);
    /// let result = pos + (1, -1);
    /// assert_eq!(result, Position::new(3, 2));
    /// ```
    fn add(self, rhs: (i32, i32)) -> Self::Output { 
        Self(IVec2::new(self.0.x + rhs.0, self.0.y + rhs.1)) 
    }
}

impl AddAssign<(i32, i32)> for Position {
    /// Adds the values of a 2D integer tuple to this position in place.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut pos = Position::new(2, 3);
    /// pos += (4, -1);
    /// assert_eq!((pos.x(), pos.y()), (6, 2));
    /// ```
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.0.x += rhs.0;
        self.0.y += rhs.1;
    }
}

/// Description component for entities
#[derive(Component, Reflect, Default, Debug, Clone, Deref, DerefMut)]
#[reflect(Component)]
pub struct Description(pub String);

impl Description {
    /// Creates a new `Description` component from any value that can be converted to a string.
    ///
    /// # Examples
    ///
    /// ```
    /// let desc = Description::new("A mysterious artifact");
    /// assert_eq!(&*desc, "A mysterious artifact");
    /// ```
    pub fn new(description: impl ToString) -> Self {
        Self(description.to_string())
    }
}

/// Component for entities that can see (field of view)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ViewShed {
    pub radius: i32,
}

impl ViewShed {
    /// Creates a new `ViewShed` component with the specified radius.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `radius` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// let viewshed = ViewShed::new(8);
    /// assert_eq!(viewshed.radius, 8);
    /// ```
    pub fn new(radius: i32) -> Self {
        debug_assert!(radius >= 0, "ViewShed radius must be non-negative");
        Self { radius }
    }
}
