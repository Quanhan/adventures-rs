use specs::{self, Join};

pub struct Room {
    pub idname: String,
    pub name: String,
    pub desc: String,
}

impl Room {
    pub fn new(n_idname: String, n_name: String, n_desc: String) -> Room {
        Room {
            idname: n_idname,
            name: n_name,
            desc: n_desc,
        }
    }
}

impl specs::Component for Room {
    type Storage = specs::VecStorage<Room>;
}

pub struct Exits {
    pub north: i32,
    pub south: i32,
    pub east: i32,
    pub west: i32,
}

impl Exits {
    pub fn new(n:i32, s:i32, e:i32, w:i32) -> Exits {
        Exits {
            north: n,
            south: s,
            east: e,
            west: w,
        }
    }
}

impl specs::Component for Exits {
    type Storage = specs::VecStorage<Exits>;
}