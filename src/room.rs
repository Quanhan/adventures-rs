use specs::{self};

pub struct Room {
    pub idname: String,
    pub name: String,
    pub desc: String,
    pub exits: Vec<Exit>,
}

impl Room {
    pub fn new(n_idname: String, n_name: String, n_desc: String, n_exits: Vec<Exit>) -> Room {
    //pub fn new(n_idname: String, n_name: String, n_desc: String) -> Room {
        Room {
            idname: n_idname,
            name: n_name,
            desc: n_desc,
            exits: n_exits,
        }
    }
}

impl specs::Component for Room {
    type Storage = specs::VecStorage<Room>;
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    East,
    West,
    Up,
    Down,
    In,
    Out,
}

pub struct Exit {
    pub direction: Direction,
    pub desternation: String,
}

impl Exit {
    pub fn new(dir: Direction, des: String) -> Exit {
        Exit {
            direction: dir,
            desternation: des,
        }
    }
}

impl specs::Component for Exit {
    type Storage = specs::VecStorage<Exit>;
}