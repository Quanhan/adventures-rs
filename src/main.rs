extern crate specs;

use std::io::{self, BufRead};

mod room;

fn main() {
    print!("{}[2J", 27 as char);
    println!("Welcome too <<< Adventure name here >>>.");
    let mut world = specs::World::new();
    world.register::<room::Room>();
    //world.register::<room::Exits>();
    world.create_now().with(room::Room::new("start".to_string(),"West of the house".to_string(), "You are standing in an open field west of a white house, with a boarded front door. You could circle the house to the north or south. There is a small mailbox here.".to_string())).build();
    
    let mut planner = specs::Planner::<()>::new(world, 4);
    
    let mut running = true;
    
    /*let (entities, rooms) = arg.fetch(|w| {
        (w.entities(), w.read::<room::Room>())
    });*/
    
    while running {
    
        println!("What now ?: ");
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines().fuse();
        let input = match lines.next() {
            Some(Ok(a)) => a,
            _ => panic!("Couldn't read input.")
        };
        
        match input.as_ref() {
            "quit" => { running = false;
                        println!("Quitting"); 
            },
            _ => println!("Huh?"),
        }
    }
}
