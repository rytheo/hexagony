use std::fmt;

use crate::coords::PointAxial;
use Direction::*;
use Redirect::*;

/// Subset of instructions that change the direction of the current IP.
#[derive(Clone, Copy)]
pub enum Redirect {
    MirrorHori,
    MirrorVert,
    MirrorForw,
    MirrorBack,
    BranchLeft,
    BranchRight,
}

/// Possible directions of travel for each IP.
#[derive(Clone, Copy)]
pub enum Direction {
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
    East,
}

impl Direction {
    /// Returns a `PointAxial` representing one grid space of movement
    /// in a given direction.
    pub fn to_vector(&self) -> PointAxial {
        match self {
            NorthEast => PointAxial(1, -1),
            NorthWest => PointAxial(0, -1),
            West => PointAxial(-1, 0),
            SouthWest => PointAxial(-1, 1),
            SouthEast => PointAxial(0, 1),
            East => PointAxial(1, 0),
        }
    }
}

/// Returns a reflected `Direction` based on which `Redirect` it hit
/// and whether the current memory edge is positive.
pub fn redirect(dir: Direction, redir: Redirect, positive: bool) -> Direction {
    match (dir, redir) {
        (NorthEast, MirrorHori) => SouthEast,
        (NorthEast, MirrorVert) => NorthWest,
        (NorthEast, MirrorForw) => NorthEast,
        (NorthEast, MirrorBack) => West,
        (NorthEast, BranchLeft) => SouthWest,
        (NorthEast, BranchRight) => East,
        (NorthWest, MirrorHori) => SouthWest,
        (NorthWest, MirrorVert) => NorthEast,
        (NorthWest, MirrorForw) => East,
        (NorthWest, MirrorBack) => NorthWest,
        (NorthWest, BranchLeft) => West,
        (NorthWest, BranchRight) => SouthEast,
        (West, MirrorHori) => West,
        (West, MirrorVert) => East,
        (West, MirrorForw) => SouthEast,
        (West, MirrorBack) => NorthEast,
        (West, BranchLeft) => East,
        (West, BranchRight) => if positive { NorthWest } else { SouthWest },
        (SouthWest, MirrorHori) => NorthWest,
        (SouthWest, MirrorVert) => SouthEast,
        (SouthWest, MirrorForw) => SouthWest,
        (SouthWest, MirrorBack) => East,
        (SouthWest, BranchLeft) => West,
        (SouthWest, BranchRight) => NorthEast,
        (SouthEast, MirrorHori) => NorthEast,
        (SouthEast, MirrorVert) => SouthWest,
        (SouthEast, MirrorForw) => West,
        (SouthEast, MirrorBack) => SouthEast,
        (SouthEast, BranchLeft) => NorthWest,
        (SouthEast, BranchRight) => East,
        (East, MirrorHori) => East,
        (East, MirrorVert) => West,
        (East, MirrorForw) => NorthWest,
        (East, MirrorBack) => SouthWest,
        (East, BranchLeft) => if positive { SouthEast } else { NorthEast },
        (East, BranchRight) => West,
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            NorthEast => "NE",
            NorthWest => "NW",
            West => "W",
            SouthWest => "SW",
            SouthEast => "SE",
            East => "E",
        })
    }
}
