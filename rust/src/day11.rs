use crate::util::Matrix;
use itertools::iproduct;

#[derive(PartialEq, Eq, Clone)]
enum Seat {
    Occupied,
    Unoccupied,
    NoSeat,
}

impl Seat {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Seat::Unoccupied,
            '#' => Seat::Occupied, //technically there shouldn't be any in input
            '.' => Seat::NoSeat,
            c => panic!("{}", c),
        }
    }

    fn is_occupied(&self) -> bool {
        match self {
            Seat::Occupied => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for Seat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Seat::*;
        match self {
            Occupied => write!(fmt, "#"),
            Unoccupied => write!(fmt, "L"),
            NoSeat => write!(fmt, "_"),
        }
    }
}

enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    fn base_offset(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            UpLeft => (-1, -1),
            Up => (0, -1),
            UpRight => (1, -1),
            Left => (-1, 0),
            Right => (1, 0),
            DownLeft => (-1, 1),
            Down => (0, 1),
            DownRight => (1, 1),
        }
    }

    fn steps_within_grid(
        &self,
        start: (i32, i32),
        width: i32,
        height: i32,
    ) -> impl Iterator<Item = (usize, usize)> {
        let (dx, dy) = self.base_offset();
        (1..)
            .map(move |step| (start.0 + (dx * step), start.1 + (dy * step)))
            .take_while(move |&(new_x, new_y)| {
                new_x >= 0 && new_y >= 0 && new_x < width && new_y < height
            })
            .map(|(x, y)| (x as usize, y as usize))
    }
}

const DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::Up,
    Direction::UpRight,
    Direction::Left,
    Direction::Right,
    Direction::DownLeft,
    Direction::Down,
    Direction::DownRight,
];

struct SeatGrid {
    seats: Matrix<Seat>,
    limited_visibility: bool,
    working_buffer: Vec<Seat>,
    // width and height stored as signed integers for convenience
    // in working with offsets
    width: i32,
    height: i32,
}
impl SeatGrid {
    fn parse(input: &str, limited_visibility: bool) -> Self {
        let width = input.lines().next().unwrap().trim().len();
        let grid: Vec<_> = input
            .lines()
            .flat_map(|l| l.trim().chars())
            .map(Seat::from_char)
            .collect();
        let seats = Matrix::wrap(grid, width);
        let (width, height) = (seats.width() as i32, seats.height() as i32);
        let working_buffer = Vec::with_capacity(seats.inner().len());
        Self {
            seats,
            limited_visibility,
            working_buffer,
            width,
            height,
        }
    }

    fn count_immediate_neighbors(&self, pos: (i32, i32)) -> usize {
        DIRECTIONS
            .iter()
            .filter_map(|dir| dir.steps_within_grid(pos, self.width, self.height).next())
            .filter(|&pos| self.seats[pos].is_occupied())
            .count()
    }

    fn count_visible_neighbors(&self, pos: (i32, i32)) -> usize {
        DIRECTIONS
            .iter()
            .filter_map(|dir| {
                dir.steps_within_grid(pos, self.width, self.height)
                    .find(|&pos| self.seats[pos] != Seat::NoSeat)
            })
            .filter(|&pos| self.seats[pos].is_occupied())
            .count()
    }

    fn simulation_step(&mut self) -> bool {
        let mut changed = false;
        let seats = self.seats.iter();
        for (pos, curr) in iproduct!(0..self.height, 0..self.width).zip(seats) {
            // (y,x) -> (x,y)
            let pos = (pos.1, pos.0);
            let occupied_neighbors = if self.limited_visibility {
                self.count_immediate_neighbors(pos)
            } else {
                self.count_visible_neighbors(pos)
            };
            // println!("{:?} {:?} {}", pos, curr, occupied_neighbors);
            let new = match (curr, occupied_neighbors) {
                (Seat::Occupied, n) if (self.limited_visibility && n >= 4) || n >= 5 => {
                    changed = true;
                    Seat::Unoccupied
                }
                (Seat::Unoccupied, 0) => {
                    changed = true;
                    Seat::Occupied
                }
                (s, _) => s.clone(),
            };
            self.working_buffer.push(new);
        }
        self.seats.clear();
        self.seats.append(&mut self.working_buffer);
        changed
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter().filter(|s| s.is_occupied()).count()
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid = SeatGrid::parse(input, true);
    while grid.simulation_step() {}
    grid.count_occupied()
}

pub fn part2(input: &str) -> usize {
    let mut grid = SeatGrid::parse(input, false);
    while grid.simulation_step() {
        // print_grid(&grid.seats)
    }
    grid.count_occupied()
}

fn print_grid(grid: &Matrix<Seat>) {
    println!("---");
    for (i, s) in grid.inner().iter().enumerate() {
        print!("{:?}", s);
        if (i + 1) % grid.width() == 0 {
            println!();
        }
    }
    println!("");
}
