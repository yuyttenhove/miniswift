
#[derive(Debug, Clone, Copy)]
pub enum Direction{
    RightDown,
    Right,
    RightUp,
    Up,
    None,
    Down,
    LeftDown,
    Left,
    LeftUp
}

pub fn direction_to_sort_list_id(dir: Direction) -> usize {
    match dir {
        Direction::RightDown => 0,
        Direction::Right => 1,
        Direction::RightUp => 2,
        Direction::Up => 3,
        Direction::None => 4,
        Direction::Down => 5,
        Direction::LeftDown => 6,
        Direction::Left => 7,
        Direction::LeftUp => 8,
    }
}

pub fn invert_direction(dir: Direction) -> Direction {
    match dir {
        Direction::RightDown => Direction::LeftUp,
        Direction::Right => Direction::Left,
        Direction::RightUp => Direction::LeftDown,
        Direction::Up => Direction::Down,
        Direction::None => Direction::None,
        Direction::Down => Direction::Up,
        Direction::LeftDown => Direction::RightUp,
        Direction::Left => Direction::Right,
        Direction::LeftUp => Direction::RightDown,
    }
}

pub fn direction_as_vector(dir: Direction) -> [f64; 2] {
    match dir {
        Direction::RightDown => [1., -1.],
        Direction::Right => [1., 0.],
        Direction::RightUp => [1., 1.],
        Direction::Up => [0., 1.],
        Direction::None => [0., 0.],
        Direction::Down => [0., -1.],
        Direction::LeftDown => [-1., -1.],
        Direction::Left => [-1., 0.],
        Direction::LeftUp => [-1., 1.],
    }
}

pub fn get_direction(anchor_i: [f64; 2], anchor_j: [f64; 2]) -> Direction {
    let left = anchor_i[0] > anchor_j[0];
    let right = anchor_i[0] < anchor_j[0];
    let down = anchor_i[1] > anchor_j[1];
    let up = anchor_i[1] < anchor_j[1];
    if left {
        if down {
            Direction::LeftDown
        } else if up {
            Direction::LeftUp
        } else {
            Direction::Left
        }
    } else if right {
        if down {
            Direction::RightDown
        } else if up {
            Direction::RightUp
        } else {
            Direction::Right
        }
    } else {
        if down {
            Direction::Down
        } else if up {
            Direction::Up
        } else {
            Direction::None
        }
    }
}