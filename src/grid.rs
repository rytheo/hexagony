use std::{fmt, str::FromStr};

use crate::{Error, coords::PointAxial, direction::Redirect};
use Op::*;

/// A pointy-topped hexagonal grid of instructions.
pub struct Grid {
    size: usize,
    grid: Vec<Vec<(Op, bool)>>,
}

impl Grid {
    /// Creates an empty `Grid` of the given side length.
    pub fn new(size: usize) -> Grid {
        let diameter = 2 * size - 1;
        let s = size - 1;
        let grid = (0..diameter).map(|i| {
            let b = if s > i { s - i } else { i - s };
            vec![(Nop, false); diameter - b]
        }).collect();
        Grid { size, grid }
    }

    /// Returns the side length of the grid.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the `Op` and debug flag at the given grid coordinates.
    pub fn get(&self, coords: PointAxial) -> (Op, bool) {
        let (row, col) = self.axial_to_index(coords);
        self.grid[row][col]
    }

    /// Converts a `PointAxial` to its corresponding internal 2D grid index.
    fn axial_to_index(&self, coords: PointAxial) -> (usize, usize) {
        let PointAxial(q, r) = coords;
        let size = self.size as isize;
        let row = r + size - 1;
        let col = q + row.min(size - 1);
        (row as usize, col as usize)
    }
}

/// Enumeration of all commands.
#[derive(Clone, Copy)]
pub enum Op {
    /// Does nothing
    Nop,
    /// Terminates the program
    Terminate,
    /// Sets the current edge to some letter's ASCII code
    Letter(u8),
    /// Multiplies the current edge by 10, then adds some digit
    Digit(u8),
    /// Increments the current edge
    Increment,
    /// Decrements the current edge
    Decrement,
    /// Sets the current edge to the sum of the left and right neighbours
    Add,
    /// Sets the current edge to the difference of the left and right neighbours (`left - right`)
    Subtract,
    /// Sets the current edge to the product of the left and right neighbours
    Multiply,
    /// Sets the current edge to the quotient of the left and right neighbours (`left / right`), rounded down
    Divide,
    /// Sets the current edge to the modulo of the left and right neighbours (`left % right`);
    /// the result will have the same sign as `right`
    Modulo,
    /// Multiplies the current edge by -1
    Negate,
    /// Reads a byte from STDIN and saves it to the current memory edge (-1 once EOF is reached)
    ReadByte,
    /// Reads and discards from STDIN until a digit, `-` or `+` is found, then reads as many bytes as possible
    /// to form a valid (signed) decimal integer and sets the current memory edge to its value
    /// (0 if EOF is reached without finding a valid number)
    ReadInt,
    /// Writes the current memory edge (mod 256) to STDOUT as a byte
    WriteByte,
    /// Writes the current memory edge's decimal representation to STDOUT
    WriteInt,
    /// Skips the next instruction
    Jump,
    /// Changes the direction of the IP
    Redir(Redirect),
    /// Switches to the previous IP (wrapping from 0 to 5)
    IPPrev,
    /// Switches to the next IP (wrapping from 5 to 0)
    IPNext,
    /// Switches to the IP with the index of the current memory edge mod 6
    IPSelect,
    /// Moves the MP to the left neighbour
    MPLeft,
    /// Moves the MP to the right neighbour
    MPRight,
    /// Moves the MP backwards to the left
    MPBackLeft,
    /// Moves the MP backwards to the right
    MPBackRight,
    /// Reverses the direction of the MP
    MPReverse,
    /// Moves the MP to the right neighbour if the current edge is positive and the left neighbour otherwise
    MPBranch,
    /// Sets the current edge to the value of the right neighbour if the current edge is positive,
    /// and the value of the left neighbour otherwise
    MemCopy,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        // Find the size of the smallest regular hexagon that will contain the code
        let src_size = s.chars().filter(|&c| !c.is_whitespace() && c != '`').count();
        let size = (1..).find(|n| 3 * n * (n - 1) + 1 >= src_size).unwrap();
        let mut grid = Grid::new(size);
        // Parse code into commands and write each command into the grid
        let mut row = 0;
        let mut col = 0;
        let mut debug = false;
        for c in s.chars() {
            let op = match c {
                _ if c.is_whitespace() => continue,
                '`' => { debug = true; continue }
                '.' => Nop,
                '@' => Terminate,
                'a'..='z' | 'A'..='Z' => Letter(c as u8),
                '0'..='9' => Digit(c as u8 - b'0'),
                ')' => Increment,
                '(' => Decrement,
                '+' => Add,
                '-' => Subtract,
                '*' => Multiply,
                ':' => Divide,
                '%' => Modulo,
                '~' => Negate,
                ',' => ReadByte,
                '?' => ReadInt,
                ';' => WriteByte,
                '!' => WriteInt,
                '$' => Jump,
                '_' => Redir(Redirect::MirrorHori),
                '|' => Redir(Redirect::MirrorVert),
                '/' => Redir(Redirect::MirrorForw),
                '\\' => Redir(Redirect::MirrorBack),
                '<' => Redir(Redirect::BranchLeft),
                '>' => Redir(Redirect::BranchRight),
                '[' => IPPrev,
                ']' => IPNext,
                '#' => IPSelect,
                '{' => MPLeft,
                '}' => MPRight,
                '"' => MPBackLeft,
                '\'' => MPBackRight,
                '=' => MPReverse,
                '^' => MPBranch,
                '&' => MemCopy,
                _ => return Err(Error::SyntaxError(c)),
            };
            grid.grid[row][col] = (op, debug);
            debug = false;
            if col < grid.grid[row].len() - 1 {
                col += 1;
            } else {
                row += 1;
                col = 0;
            }
        }
        Ok(grid)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.grid {
            // Pad lines with whitespace for hex shape
            write!(f, "{}", " ".repeat(2 * self.size - 1 - line.len()))?;
            for (op, dbg) in line {
                write!(f, "{}{}", if *dbg { '`' } else { ' ' }, op)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}


impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Nop => '.',
            Terminate => '@',
            Letter(b) => *b as char,
            Digit(d) => (*d + b'0') as char,
            Increment => ')',
            Decrement => '(',
            Add => '+',
            Subtract => '-',
            Multiply => '*',
            Divide => ':',
            Modulo => '%',
            Negate => '~',
            ReadByte => ',',
            ReadInt => '?',
            WriteByte => ';',
            WriteInt => '!',
            Jump => '$',
            Redir(Redirect::MirrorHori) => '_',
            Redir(Redirect::MirrorVert) => '|',
            Redir(Redirect::MirrorForw) => '/',
            Redir(Redirect::MirrorBack) => '\\',
            Redir(Redirect::BranchLeft) => '<',
            Redir(Redirect::BranchRight) => '>',
            IPPrev => '[',
            IPNext => ']',
            IPSelect => '#',
            MPLeft => '{',
            MPRight => '}',
            MPBackLeft => '"',
            MPBackRight => '\'',
            MPReverse => '=',
            MPBranch => '^',
            MemCopy => '&',
        })
    }
}
