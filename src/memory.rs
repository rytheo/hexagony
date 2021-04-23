use std::collections::HashMap;
use std::fmt;
use rug::Integer;

/// One of three edges of the hex used for indexing.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    NE,
    E,
    SE,
}

/// Orientation of a memory pointer relative to its hex.
#[derive(Clone, Copy)]
enum Rot {
    CW,
    CCW,
}

/// Tuple of values used as the index of a memory edge.
type Index = (isize, isize, Dir);

/// A pointy-topped hexagonal grid that stores an integer in each edge.
///
/// Edges are indexed by the axial coordinates of the westward adjacent hexagon,
/// and a direction (NE, E, SE) to identify a specific edge of the hexagon.
pub struct Memory {
    mem: HashMap<Index, Integer>,
    mp: Index,
    rot: Rot,
    default: Integer,
}

impl Memory {
    /// Creates an empty `Memory` instance.
    pub fn new() -> Memory {
        Memory {
            mem: HashMap::new(),
            mp: (0, 0, Dir::E),
            rot: Rot::CCW,
            default: Integer::new(),
        }
    }

    /// Returns the index of the left neighbour edge.
    fn left_index(&self) -> (Index, Rot) {
        let (q, r, e) = self.mp;
        match (e, self.rot) {
            (Dir::NE, Rot::CCW) => ((q, r - 1, Dir::SE), Rot::CW),
            (Dir::NE, Rot::CW) => ((q + 1, r - 1, Dir::SE), Rot::CCW),
            (Dir::E, Rot::CCW) => ((q, r, Dir::NE), Rot::CCW),
            (Dir::E, Rot::CW) => ((q, r + 1, Dir::NE), Rot::CW),
            (Dir::SE, Rot::CCW) => ((q, r, Dir::E), Rot::CCW),
            (Dir::SE, Rot::CW) => ((q - 1, r + 1, Dir::E), Rot::CW),
        }
    }

    /// Returns the index of the right neighbour edge.
    fn right_index(&self) -> (Index, Rot) {
        let (q, r, e) = self.mp;
        match (e, self.rot) {
            (Dir::NE, Rot::CCW) => ((q, r - 1, Dir::E), Rot::CCW),
            (Dir::NE, Rot::CW) => ((q, r, Dir::E), Rot::CW),
            (Dir::E, Rot::CCW) => ((q + 1, r - 1, Dir::SE), Rot::CCW),
            (Dir::E, Rot::CW) => ((q, r, Dir::SE), Rot::CW),
            (Dir::SE, Rot::CCW) => ((q, r + 1, Dir::NE), Rot::CW),
            (Dir::SE, Rot::CW) => ((q - 1, r + 1, Dir::NE), Rot::CCW),
        }
    }

    /// Returns a reference to the value in the left neighbour.
    pub fn get_left(&self) -> &Integer {
        self.mem.get(&self.left_index().0).unwrap_or(&self.default)
    }

    /// Returns a reference to the value in the right neighbour.
    pub fn get_right(&self) -> &Integer {
        self.mem.get(&self.right_index().0).unwrap_or(&self.default)
    }

    /// Returns a reference to the value in the current memory edge.
    pub fn get(&self) -> &Integer {
        self.mem.get(&self.mp).unwrap_or(&self.default)
    }

    /// Sets the current memory edge to the given value.
    pub fn set(&mut self, value: Integer) {
        self.mem.insert(self.mp, value);
    }

    /// Returns a mutable reference to the value in the current memory edge.
    pub fn get_mut(&mut self) -> &mut Integer {
        self.mem.entry(self.mp).or_default()
    }

    /// Moves the MP to the left neighbour.
    pub fn move_left(&mut self) {
        let (mp, rot) = self.left_index();
        self.mp = mp;
        self.rot = rot;
    }

    /// Moves the MP to the right neighbour.
    pub fn move_right(&mut self) {
        let (mp, rot) = self.right_index();
        self.mp = mp;
        self.rot = rot;
    }

    /// Reverses the direction of the MP.
    pub fn reverse(&mut self) {
        self.rot = match self.rot {
            Rot::CW => Rot::CCW,
            Rot::CCW => Rot::CW,
        };
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Dir::NE => "NE",
            Dir::E => "E",
            Dir::SE => "SE",
        })
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ((q, r, d), v) in &self.mem {
            writeln!(f, "({}, {}, {}): {}", q, r, d, v)?;
        }
        Ok(())
    }
}
