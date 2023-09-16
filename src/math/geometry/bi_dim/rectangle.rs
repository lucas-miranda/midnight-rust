use num_traits::Num;

use super::Vector2;

pub type Rect<T> = Rectangle<T>;
pub type Position<T> = Vector2<T>;
pub type Size<T> = Vector2<T>;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rectangle<T> where
    T: Num
{
    pub position: Position<T>,
    pub size: Size<T>,
}

impl<T> Rectangle<T> where
    T: Num
{
    pub const fn new(position: Position<T>, size: Size<T>) -> Self {
        Self {
            position,
            size,
        }
    }
}

impl<T> Rectangle<T> where
    T: Num + Clone + Copy
{
    pub fn points(top_left: Position<T>, bottom_right: Position<T>) -> Self {
        Self {
            position: top_left,
            size: bottom_right - top_left,
        }
    }

    pub const fn left(&self) -> T {
        self.position.x
    }

    pub const fn top(&self) -> T {
        self.position.y
    }

    pub fn right(&self) -> T {
        self.position.x + self.size.x
    }

    pub fn bottom(&self) -> T {
        self.position.y + self.size.y
    }

    pub const fn top_left(&self) -> Position<T> {
        self.position
    }

    pub fn top_right(&self) -> Position<T> {
        Vector2::new(self.right(), self.top())
    }

    pub fn bottom_right(&self) -> Position<T> {
        Vector2::new(self.right(), self.bottom())
    }

    pub fn bottom_left(&self) -> Position<T> {
        Vector2::new(self.left(), self.bottom())
    }
}
