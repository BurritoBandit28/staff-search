use std::ops::{Add, AddAssign, Mul};


// I never got far enough to use this but it was gonna be funny
#[derive(Clone)]
pub struct Vector2<T : Mul + Add + Copy + AddAssign> {

    x : T,
    y : T

}

impl<T : Mul<Output = T> + Add + Copy + std::ops::AddAssign> Mul for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T : Mul + Add<Output = T> + Copy + std::ops::AddAssign> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Vector2<T> {
        Vector2 {
            x : self.x + rhs.x,
            y : self.y + rhs.y
        }
    }
}

impl<T : Mul + Add + Copy + std::ops::AddAssign> Vector2<T> {
    pub fn get_x(&self) -> T{
        self.x
    }
    pub fn get_y(&self) -> T {
        self.y
    }

    pub fn set_x(&mut self, amnt: T) {
        self.x = amnt
    }
    pub fn set_y(&mut self, amnt: T) {
        self.y = amnt
    }

    pub fn change_x(&mut self, amnt: T) {
        self.x += amnt
    }
    pub fn change_y(&mut self, amnt: T) {
        self.y += amnt
    }

    pub fn new(x : T, y : T) -> Self {
        Self {
            x,y
        }
    }

}