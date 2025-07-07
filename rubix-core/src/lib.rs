#[derive(Debug, Copy, Clone)]
enum FaceColor {
    White,
    Red,
    Green,
    Orange,
    Blue,
    Yellow,
}
#[derive(Debug, Copy, Clone)]
enum FaceOrientation {
    Top,

    Front,
    Left,
    Back,
    Right,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
struct Face([FaceColor; 9]);
#[derive(Debug, Clone, Copy)]
struct Cube([Face; 6]);
impl Cube {
    /// Creates a new [`Cube`]. The cube will be fully solved,
    /// with the faces in the following arrangement
    /// - Top: White
    /// - Bottom: Yellow
    /// - Front: Red
    /// - Left: Green
    /// - Back: Orange
    /// - Right: Blue
    pub const fn new() -> Self {
        let top_face = Face([FaceColor::White; 9]);
        let front_face = Face([FaceColor::White; 9]);
        let left_face = Face([FaceColor::White; 9]);
        let back_face = Face([FaceColor::White; 9]);
        let right_face = Face([FaceColor::White; 9]);
        let bottom_face = Face([FaceColor::White; 9]);

        Self([
            top_face,
            front_face,
            left_face,
            back_face,
            right_face,
            bottom_face,
        ])
    }

    pub fn face(&self, direction: FaceOrientation) -> &Face {
        &self.0[direction as usize]
    }

    pub fn apply(&mut self, change: Move) {
        // TODO: Apply the move
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: MoveDirection,
    face: AffectedFace,
}
#[derive(Debug, Clone, Copy)]
enum MoveDirection {
    Clockwise,
    CounterClockwise,
}
#[derive(Debug, Clone, Copy)]
enum AffectedFace {
    TopRow,
    MiddleRow,
    BottomRow,
    RightColumn,
    MiddleColumn,
    LeftColumn,
    Front,
    Back,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
