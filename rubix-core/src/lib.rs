const RED_ADJACENCY: [FaceColor; 4] = [
    FaceColor::Green,
    FaceColor::White,
    FaceColor::Blue,
    FaceColor::Yellow,
];

const GREEN_ADJACENCY: [FaceColor; 4] = [
    FaceColor::Orange,
    FaceColor::White,
    FaceColor::Red,
    FaceColor::Yellow,
];

const ORANGE_ADJACENCY: [FaceColor; 4] = [
    FaceColor::Blue,
    FaceColor::White,
    FaceColor::Green,
    FaceColor::Yellow,
];

const BLUE_ADJACENCY: [FaceColor; 4] = [
    FaceColor::Red,
    FaceColor::White,
    FaceColor::Orange,
    FaceColor::Yellow,
];

const WHITE_ADJACENCY: [FaceColor; 4] = [
    FaceColor::Green,
    FaceColor::Orange,
    FaceColor::Blue,
    FaceColor::Red,
];

const YELLOW_ADJACENCY: [FaceColor; 4] = [
    FaceColor::Green,
    FaceColor::Red,
    FaceColor::Blue,
    FaceColor::Orange,
];

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
struct Face {
    center: FaceColor,
    vertices: [FaceColor; 8],
}
impl Face {
    pub const fn new(color: FaceColor) -> Self {
        Self {
            center: color,
            vertices: [color; 8],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    faces: [Face; 6],
    front: FaceColor,
    top: FaceColor,
}
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
        let top_face = Face::new(FaceColor::White);
        let front_face = Face::new(FaceColor::Red);
        let left_face = Face::new(FaceColor::Green);
        let back_face = Face::new(FaceColor::Orange);
        let right_face = Face::new(FaceColor::Blue);
        let bottom_face = Face::new(FaceColor::Yellow);

        Self {
            faces: [
                top_face,
                front_face,
                left_face,
                back_face,
                right_face,
                bottom_face,
            ],
            front: FaceColor::Red,
            top: FaceColor::White,
        }
    }

    pub fn face(&self, direction: FaceOrientation) -> &Face {
        match direction {
            FaceOrientation::Top => self.color_face(self.top),
            FaceOrientation::Front => self.color_face(self.front),
            FaceOrientation::Left => todo!(),
            FaceOrientation::Back => todo!(),
            FaceOrientation::Right => todo!(),
            FaceOrientation::Bottom => todo!(),
        }
    }

    pub fn color_face(&self, color: FaceColor) -> &Face {
        &self.faces[color as usize]
    }

    pub fn apply(&mut self, change: Move) {
        // TODO: Apply the move
    }
}

// TODO: There should be a distinction of face-moves and cube-moves
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
