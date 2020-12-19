enum Action {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    TurnLeft,
    TurnRight,
    GoForward,
}

struct Instruction {
    action: Action,
    value: i32,
}

impl Instruction {
    fn parse(raw: &str) -> Option<Self> {
        use Action::*;
        let (action, value) = raw.split_at(1);
        let action = match action {
            "N" => MoveNorth,
            "S" => MoveSouth,
            "E" => MoveEast,
            "W" => MoveWest,
            "L" => TurnLeft,
            "R" => TurnRight,
            "F" => GoForward,
            _ => return None,
        };
        let value = value.parse().ok()?;
        Some(Instruction { action, value })
    }
}

//                    N
//                 (0,  1)
//
// W (-1, 0)  start(0,0)-facing>  (1, 0) E
//
//                 (0, -1)
//                    S
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn left(&mut self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
    fn right(&mut self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn movement_offset(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (0, 1),
            South => (0, -1),
            West => (-1, 0),
            East => (1, 0),
        }
    }
}

fn manhattan_from_start((a_x, a_y): (i32, i32)) -> i32 {
    a_x.abs() + a_y.abs()
}

pub fn part1(input: &str) -> i32 {
    let instructions = input.lines().map(|l| Instruction::parse(l).unwrap());
    let mut ship_pos = (0, 0);
    let mut facing = Direction::East;
    for inst in instructions {
        use Action::*;
        match inst.action {
            MoveNorth => ship_pos.1 += inst.value,
            MoveSouth => ship_pos.1 -= inst.value,
            MoveEast => ship_pos.0 += inst.value,
            MoveWest => ship_pos.0 -= inst.value,
            TurnLeft => (0..(inst.value / 90)).for_each(|_| facing = facing.left()),
            TurnRight => (0..(inst.value / 90)).for_each(|_| facing = facing.right()),
            GoForward => {
                let (dx, dy) = facing.movement_offset();
                ship_pos.0 += dx * inst.value;
                ship_pos.1 += dy * inst.value
            }
        }
    }
    manhattan_from_start(ship_pos)
}

struct Waypoint(i32, i32);
// (WE, NS)
// (10, 4) -L-> (-4,  10) -> (-10, -4) -> ( 4, -10) -> (10, 4)
// (10, 4) -R-> ( 4, -10) -> (-10, -4) -> (-4,  10) -> (10, 4)
impl Waypoint {
    fn left(&mut self) {
        let (we, ns) = (-self.1, self.0);
        self.0 = we;
        self.1 = ns;
    }

    fn right(&mut self) {
        let (we, ns) = (self.1, -self.0);
        self.0 = we;
        self.1 = ns;
    }
}

pub fn part2(input: &str) -> i32 {
    let instructions = input.lines().map(|l| Instruction::parse(l).unwrap());
    let mut ship_pos = (0, 0);
    let mut waypoint = Waypoint(10, 1);
    for inst in instructions {
        use Action::*;
        match inst.action {
            MoveNorth => waypoint.1 += inst.value,
            MoveSouth => waypoint.1 -= inst.value,
            MoveEast => waypoint.0 += inst.value,
            MoveWest => waypoint.0 -= inst.value,
            TurnLeft => (0..(inst.value / 90)).for_each(|_| waypoint.left()),
            TurnRight => (0..(inst.value / 90)).for_each(|_| waypoint.right()),
            GoForward => {
                ship_pos.0 += waypoint.0 * inst.value;
                ship_pos.1 += waypoint.1 * inst.value;
            }
        }
    }
    manhattan_from_start(ship_pos)
}
