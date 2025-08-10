use std::{
    collections::LinkedList,
    fmt::{Debug, Display},
};

use arbitrary_int::{Number, u3, u9, u24};
use bitbybit::{bitenum, bitfield};
use colored::Colorize;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FaceEdge {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FaceRow {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, PartialEq, Eq)]
struct LinkedListFace {
    center_color: FaceColor,
    vertex_colors: LinkedList<FaceColor>,
}

impl Display for LinkedListFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = "■";
        let as_colored = |vertex| match vertex {
            FaceColor::White => data.white(),
            FaceColor::Red => data.red(),
            FaceColor::Green => data.green(),
            FaceColor::Orange => data.magenta(),
            FaceColor::Blue => data.blue(),
            FaceColor::Yellow => data.yellow(),
        };
        let colors = self
            .vertex_colors
            .iter()
            .map(|vertex| as_colored(*vertex))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let [
            bottom_left,
            left,
            top_left,
            top,
            top_right,
            right,
            bottom_right,
            bottom,
        ] = colors;

        write!(
            f,
            "{top_left} {top} {top_right}\n{left} X {right}\n{bottom_left} {bottom} {bottom_right}"
        )
    }
}

impl LinkedListFace {
    pub fn new(color: FaceColor) -> Self {
        Self {
            center_color: color,
            vertex_colors: LinkedList::from([color; 8]),
        }
    }

    fn from(center: FaceColor, colors: [FaceColor; 8]) -> Self {
        Self {
            center_color: center,
            vertex_colors: LinkedList::from(colors),
        }
    }

    pub fn get_edge(&self, edge: FaceEdge) -> [FaceColor; 3] {
        match edge {
            FaceEdge::Left => todo!(),
            FaceEdge::Up => todo!(),
            FaceEdge::Right => todo!(),
            FaceEdge::Down => todo!(),
        }
    }

    pub fn get_row(&self, row: FaceRow) -> [FaceColor; 3] {
        todo!()
    }

    pub fn rotate_cw(&mut self) {
        for _ in 0..2 {
            let node = self.vertex_colors.pop_back().unwrap();
            self.vertex_colors.push_front(node);
        }
    }

    pub fn rotate_ccw(&mut self) {
        for _ in 0..2 {
            let node = self.vertex_colors.pop_front().unwrap();
            self.vertex_colors.push_back(node);
        }
    }

    pub fn shift_in_cw(&mut self, edge: FaceEdge, vertices: [FaceColor; 3]) {
        if edge != FaceEdge::Down {
            let mut end = match edge {
                FaceEdge::Left => self.vertex_colors.split_off(0),
                FaceEdge::Up => self.vertex_colors.split_off(2),
                FaceEdge::Right => self.vertex_colors.split_off(4),
                FaceEdge::Down => unreachable!(),
            };

            for vertex in vertices {
                self.vertex_colors.push_back(vertex);
                end.pop_front();
            }

            self.vertex_colors.append(&mut end);
        } else {
            self.vertex_colors.pop_front();
            let mut vertices = LinkedList::from(vertices);
            self.vertex_colors.push_front(vertices.pop_front().unwrap());
            self.vertex_colors.pop_back();
            self.vertex_colors.pop_back();
            self.vertex_colors.append(&mut vertices);
        }
    }

    pub fn shift_in_ccw(&mut self, edge: FaceEdge, vertices: [FaceColor; 3]) {
        todo!()
    }
}

struct LinkedListCube {
    faces: [LinkedListFace; 6],
    front: FaceColor,
    top: FaceColor,
}

impl LinkedListCube {
    pub fn new() -> Self {
        let top_face = LinkedListFace::new(FaceColor::White);
        let front_face = LinkedListFace::new(FaceColor::Red);
        let left_face = LinkedListFace::new(FaceColor::Green);
        let back_face = LinkedListFace::new(FaceColor::Orange);
        let right_face = LinkedListFace::new(FaceColor::Blue);
        let bottom_face = LinkedListFace::new(FaceColor::Yellow);

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

    pub fn apply(&mut self, change: Move) {
        todo!()
    }
}

// 000
// 001
// 010
// 011
// 100
// 101
// 110
// 111

/// The idea is to make the face as small as possible,
/// by bit-packing it's color values. The center color never
/// changes and will be defined a level higher, which leaves us
/// with 8 vertices to store the color on. A color has 6 possible
/// values, which means it fits nicely into a u3, which gives us
/// a packed representation of 3*8 = 24 bits. Would be nice to
/// be able to shave if a bit more, so we can pack the whole cube
/// in a u128 though. Currently 24 * 6 = 144 which is slightly too much
/// If we manage to pack a face in 21 bits, we are right on target and even
/// have 2 bits left to maybe store the orientation of the cube
#[bitfield(u24, default = 0)]
struct BitFace {
    #[bits(0..=2, rw)]
    vertex_colors: [u3; 8],
}

#[bitenum(u3, exhaustive = false)]
enum BitFaceColor {
    White = 0b000,
    Red = 0b001,
    Green = 0b010,
    Orange = 0b011,
    Blue = 0b100,
    Yellow = 0b101,
}

impl BitFace {
    pub fn edge(&self, edge: FaceEdge) -> u9 {
        match edge {
            // TODO: Start bits are wrong! It's index * bits somehow? 0..6 6..12 12..18 18..24
            FaceEdge::Left => u9::extract_u32(self.raw_value, 0),
            FaceEdge::Up => u9::extract_u32(self.raw_value, 6),
            FaceEdge::Right => u9::extract_u32(self.raw_value, 12),
            FaceEdge::Down => u9::extract_u32(self.raw_value, 18), // need this wrapping somehow...
        }
    }
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

    pub fn edge_mut(&mut self, edge: FaceEdge) -> [&mut FaceColor; 3] {
        todo!()
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
        let edge_loop = self.edge_loop(change.face);
        let colors = edge_loop
            .iter()
            .map(|p| **p)
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();

        match change.direction {
            MoveDirection::Clockwise => {
                for (ptr, color) in edge_loop.into_iter().zip(colors.skip(3)) {
                    *ptr = color;
                }
            }
            MoveDirection::CounterClockwise => {
                for (idx, color) in (3..=14).zip(colors) {
                    *edge_loop[idx % 12] = color;
                }
            }
        }
    }

    // TODO: I think this can not be implemented easily with the borrow checker.
    // I was thinking to move to a bit-packed representation anyways, which will
    // make this manipulation way easier
    fn edge_loop(&mut self, loop_face: AffectedFace) -> [&mut FaceColor; 12] {
        todo!()
        // [
        //     self.faces[0].edge_mut(FaceEdge::Right),
        //     self.faces[1].edge_mut(FaceEdge::Up),
        //     self.faces[2].edge_mut(FaceEdge::Up),
        //     self.faces[3].edge_mut(FaceEdge::Up),
        // ]
        // .into_iter()
        // .flatten()
        // .collect::<Vec<_>>()
        // .try_into()
        // .unwrap()
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
mod linkedlistface_tests {
    use super::*;

    #[test]
    fn can_rotate_cw() {
        let center = FaceColor::Blue;
        let colors = [
            FaceColor::White,
            FaceColor::Red,
            FaceColor::Green,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
        ];
        let mut face = LinkedListFace::from(center, colors);

        let colors = [
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::White,
            FaceColor::Red,
            FaceColor::Green,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
        ];
        let expected_face = LinkedListFace::from(center, colors);

        face.rotate_cw();

        println!("expect:\n{expected_face}");
        println!("actual:\n{face}");
        assert_eq!(face, expected_face);
    }

    #[test]
    fn can_rotate_ccw() {
        let center = FaceColor::Blue;
        let colors = [
            FaceColor::White,
            FaceColor::Red,
            FaceColor::Green,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
        ];
        let mut face = LinkedListFace::from(center, colors);

        let colors = [
            FaceColor::Green,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::White,
            FaceColor::Red,
        ];
        let expected_face = LinkedListFace::from(center, colors);

        face.rotate_ccw();

        println!("expect:\n{expected_face}");
        println!("actual:\n{face}");
        assert_eq!(face, expected_face);
    }

    #[test]
    fn can_shift_in_cw() {
        let center = FaceColor::Blue;
        let edge_colors = [FaceColor::White, FaceColor::Red, FaceColor::Green];
        let edge = FaceEdge::Up;
        let colors = [
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
        ];
        let mut face = LinkedListFace::from(center, colors);

        let colors = [
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::White,
            FaceColor::Red,
            FaceColor::Green,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
        ];
        let expected_face = LinkedListFace::from(center, colors);

        face.shift_in_cw(edge, edge_colors);

        println!("expect:\n{expected_face}");
        println!("actual:\n{face}");
        assert_eq!(face, expected_face);

        let edge_colors = [FaceColor::Yellow, FaceColor::White, FaceColor::Orange];
        let edge = FaceEdge::Left;
        let colors = [
            FaceColor::Yellow,
            FaceColor::White,
            FaceColor::Orange,
            FaceColor::Red,
            FaceColor::Green,
            FaceColor::Blue,
            FaceColor::Blue,
            FaceColor::Blue,
        ];
        let expected_face = LinkedListFace::from(center, colors);
        face.shift_in_cw(edge, edge_colors);

        println!("expect:\n{expected_face}");
        println!("actual:\n{face}");
        assert_eq!(face, expected_face);

        let edge_colors = [FaceColor::Orange, FaceColor::Yellow, FaceColor::White];
        let edge = FaceEdge::Right;
        let colors = [
            FaceColor::Yellow,
            FaceColor::White,
            FaceColor::Orange,
            FaceColor::Red,
            FaceColor::Orange,
            FaceColor::Yellow,
            FaceColor::White,
            FaceColor::Blue,
        ];
        let expected_face = LinkedListFace::from(center, colors);
        face.shift_in_cw(edge, edge_colors);

        println!("expect:\n{expected_face}");
        println!("actual:\n{face}");
        assert_eq!(face, expected_face);

        let edge_colors = [FaceColor::Red, FaceColor::Orange, FaceColor::Yellow];
        let edge = FaceEdge::Down;
        let colors = [
            FaceColor::Red,
            FaceColor::White,
            FaceColor::Orange,
            FaceColor::Red,
            FaceColor::Orange,
            FaceColor::Yellow,
            FaceColor::Orange,
            FaceColor::Yellow,
        ];
        let expected_face = LinkedListFace::from(center, colors);
        face.shift_in_cw(edge, edge_colors);

        println!("expect:\n{expected_face}");
        println!("actual:\n{face}");
        assert_eq!(face, expected_face);
    }
}
