use std::{fmt, io::{self, Read, Write}, iter::Peekable};
use rug::{Assign, Integer};

use coords::PointAxial;
use direction::{Direction, redirect};
use grid::{Grid, Op};
use memory::Memory;

mod coords;
mod direction;
mod grid;
mod memory;

/// Returns a `String` representation of an empty `Grid` with the given side length.
pub fn source_template(size: usize) -> String {
    match size {
        0 => String::new(),
        _ => Grid::new(size).to_string(),
    }
}

/// Parses and runs a string slice of Hexagony source code.
///
/// If the `debug_level` is 1, debug info will be printed when an instruction
/// with a debug flag is executed.
///
/// If the `debug_level` is 2, debug info will be printed when executing any instruction.
pub fn run(src: &str, debug_level: u8) -> Result<(), Error> {
    Hexagony::new(src, debug_level)?.run()
}

/// A Hexagony interpreter.
///
/// Stores all state-related information needed to run a Hexagony program.
struct Hexagony {
    grid: Grid,
    mem: Memory,
    ips: [IP; 6],
    ip_idx: usize,
    tick: Integer,
    debug_level: u8,
    input: Peekable<io::Bytes<io::Stdin>>,
}

/// An instruction pointer (IP).
///
/// Each IP stores its location on the grid and its current direction.
struct IP {
    coords: PointAxial,
    dir: Direction,
}

impl Hexagony {
    /// Creates a new Hexagony interpreter with the given source code and debug level.
    fn new(src: &str, debug_level: u8) -> Result<Self, Error> {
        let grid: Grid = src.parse()?;
        let size = grid.size() as isize;
        Ok(Hexagony {
            grid,
            mem: Memory::new(),
            ips: [
                IP { coords: PointAxial(0, -size + 1), dir: Direction::East },
                IP { coords: PointAxial(size - 1, -size + 1), dir: Direction::SouthEast },
                IP { coords: PointAxial(size - 1, 0), dir: Direction::SouthWest },
                IP { coords: PointAxial(0, size - 1), dir: Direction::West },
                IP { coords: PointAxial(-size + 1, size - 1), dir: Direction::NorthWest },
                IP { coords: PointAxial(-size + 1, 0), dir: Direction::NorthEast },
            ],
            ip_idx: 0,
            tick: Integer::new(),
            debug_level,
            input: std::io::stdin().bytes().peekable(),
        })
    }

    /// Runs the interpreter.
    ///
    /// Returns `Ok` if it hit a terminate instruction and `Err` if a runtime error occurred.
    fn run(&mut self) -> Result<(), Error> {
        loop {
            let (op, dbg) = self.grid.get(self.ips[self.ip_idx].coords);
            let dbg_tick = self.debug_level > 1 && dbg || self.debug_level > 0;
            if dbg_tick {
                eprintln!("\nTick {}:", self.tick);
                eprintln!("IPs (! indicates active IP): ");
                for (i, ip) in self.ips.iter().enumerate() {
                    eprintln!("{} {}: {}, {}", if self.ip_idx == i { '!' } else { ' ' }, i, ip.coords, ip.dir);
                }
                eprintln!("Command: {}", op);
            }
            let mut next_idx = self.ip_idx;
            match op {
                Op::Nop => (),
                Op::Terminate => {
                    if dbg_tick {
                        eprintln!("Memory: {}", self.mem);
                    }
                    return Ok(())
                }
                Op::Letter(b) => self.mem.get_mut().assign(b),
                Op::Digit(d) => {
                    let val = self.mem.get_mut();
                    *val *= 10;
                    *val += d;
                }
                Op::Increment => *self.mem.get_mut() += 1,
                Op::Decrement => *self.mem.get_mut() -= 1,
                Op::Add => self.mem.set((self.mem.get_left() + self.mem.get_right()).into()),
                Op::Subtract => self.mem.set((self.mem.get_left() - self.mem.get_right()).into()),
                Op::Multiply => self.mem.set((self.mem.get_left() * self.mem.get_right()).into()),
                Op::Divide => {
                    if *self.mem.get_right() == 0 { return Err(Error::ZeroDivisionError) }
                    self.mem.set((self.mem.get_left() / self.mem.get_right()).into());
                },
                Op::Modulo => self.mem.set({
                    let (left, right) = (self.mem.get_left(), self.mem.get_right());
                    if *right == 0 { return Err(Error::ZeroDivisionError) }
                    let (_, rem) = left.div_rem_ref(right).into();
                    if rem != 0 && (*left < 0) != (*right < 0) { rem + right } else { rem }
                }),
                Op::Negate => *self.mem.get_mut() *= -1,
                Op::ReadByte => self.mem.set(match self.input.next() {
                    Some(b) => Integer::from(b?),
                    None => Integer::from(-1),
                }),
                Op::ReadInt => {
                    let val = self.mem.get_mut();
                    val.assign(0);
                    let mut sign = 1;
                    while let Some(b) = self.input.next() {
                        match b? {
                            b'+' => break,
                            b'-' => {
                                sign = -1;
                                break;
                            }
                            d @ b'0'..=b'9' => {
                                *val *= 10;
                                *val += d - b'0';
                                break;
                            }
                            _ => (),
                        }
                    }
                    while let Some(Ok(d @ b'0'..=b'9')) = self.input.peek() {
                        *val *= 10;
                        *val += d - b'0';
                        self.input.next();
                    }
                    *val *= sign;
                }
                Op::WriteByte => io::stdout().write_all(&[self.mem.get().mod_u(256) as u8])?,
                Op::WriteInt => print!("{}", self.mem.get()),
                Op::Jump => self.advance_ip(),
                Op::Redir(redir) => {
                    let ip = &mut self.ips[self.ip_idx];
                    ip.dir = redirect(ip.dir, redir, *self.mem.get() > 0);
                }
                Op::IPPrev => next_idx = (self.ip_idx + 5) % 6, // +5 (= -1 mod 6) to avoid underflow
                Op::IPNext => next_idx = (self.ip_idx + 1) % 6,
                Op::IPSelect => next_idx = self.mem.get().mod_u(6) as usize,
                Op::MPLeft => self.mem.move_left(),
                Op::MPRight => self.mem.move_right(),
                Op::MPBackLeft => { self.mem.reverse(); self.mem.move_right(); self.mem.reverse(); }
                Op::MPBackRight => { self.mem.reverse(); self.mem.move_left(); self.mem.reverse(); }
                Op::MPReverse => self.mem.reverse(),
                Op::MPBranch => if *self.mem.get() > 0 { self.mem.move_right() } else { self.mem.move_left() }
                Op::MemCopy => self.mem.set(if *self.mem.get() > 0 { self.mem.get_right().clone() } else { self.mem.get_left().clone() }),
            }
            if dbg_tick {
                eprintln!("New direction: {}", self.ips[self.ip_idx].dir);
                eprintln!("Memory:\n{}", self.mem);
            }
            self.advance_ip();
            self.ip_idx = next_idx;
            self.tick += 1
        }
    }

    /// Moves the current IP to the next grid space in its current direction.
    fn advance_ip(&mut self) {
        if self.grid.size() == 1 {
            return;
        }
        let ip = &mut self.ips[self.ip_idx];
        // Use post-move cube coords to check for wrapping
        ip.coords += ip.dir.to_vector();
        let PointAxial(x, z) = ip.coords;
        let y = -x - z;
        let size = self.grid.size();
        let (x_big, y_big, z_big) = (x.abs() as usize >= size, y.abs() as usize >= size, z.abs() as usize >= size);
        // Return early if (x, y, z) are in-bounds
        if !(x_big || y_big || z_big) {
            return;
        }
        // Use pre-move axial coords to compute wrapped coords
        ip.coords -= ip.dir.to_vector();
        let PointAxial(q, r) = ip.coords;
        ip.coords = match (x_big, y_big, z_big, *self.mem.get() > 0) {
            // Impossible to be all in range or out of range here
            (false, false, false, _) | (true, true, true, _) => unreachable!(),
            // If two values are in range, wrap around an edge
            (false, false, true, _) => PointAxial(q + r, -r),
            (false, true, false, _) => PointAxial(-r, -q),
            (true, false, false, _) => PointAxial(-q, q + r),
            // If one value is in range, branch out of a corner
            // There are two paths that lead to each corner
            (false, true, true, false) | (true, false, true, true) => PointAxial(q + r, -r),
            (true, false, true, false) | (true, true, false, true) => PointAxial(-q, q + r),
            (true, true, false, false) | (false, true, true, true) => PointAxial(-r, -q),
        }
    }
}

/// Error type returned by functions in this crate.
#[derive(Debug)]
pub enum Error {
    SyntaxError(char),
    IOError(io::Error),
    ZeroDivisionError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SyntaxError(c) => write!(f, "Unrecognized character in source code: {}", c),
            Error::IOError(e) => write!(f, "{}", e),
            Error::ZeroDivisionError => write!(f, "Division by zero"),
        }
    }
}
