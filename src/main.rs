extern crate specs;

use std::io::{BufRead};
use specs::Join;

mod room;

fn main() {
    print!("{}[2J", 27 as char);
    println!("Welcome to <<< Adventure name here >>>.");
    let mut world = specs::World::new();
    world.register::<room::Room>();
    world.register::<room::Exit>();
    world.create_now().with(room::Room::new(
            "start".to_string(),"West of the house".to_string(), 
            "You are standing in an open field west of a white house, with a boarded front door. You could circle the house to the north or south. There is a small mailbox here.".to_string())
        ).with(room::Exit::new(
            room::Direction::North,
            "northhouse".to_string()
        ))
    .build();
    world.create_now().with(room::Room::new("northhouse".to_string(),"North of the house".to_string(), "This is a bit more of a breeze this side of the house but the warm summer sun is keeping from getting cold. A boarded entrence to the house is here.".to_string())).build();
    
    let mut planner = specs::Planner::<()>::new(world, 4);
    let mut running = true;
    let mut show_desc = true;
    
    let mut current_room = "start";
    
    /*let (entities, rooms) = arg.fetch(|w| {
        (w.entities(), w.read::<room::Room>())
    });*/
    
    while running {
        
        if show_desc {
            
            show_desc = false;
            planner.run_custom(move |arg| {
                let rooms = arg.fetch(|w| {
                    (w.read::<room::Room>()) 
                });
                
                for room in (rooms).iter() {
                    if current_room == room.idname {
                        println!("{}", room.name);
                        println!("{}", room.desc);
                    }
                }
            });
            planner.wait();
        }
        println!("What now ?: ");
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines().fuse();
        let input = match lines.next() {
            Some(Ok(a)) => a,
            _ => panic!("Couldn't read input.")
        };
        
        match input.as_ref() {
            "look" => show_desc = true,
            "quit" => { running = false;
                        println!("Quitting"); 
            },
            _ => println!("Huh?"),
        }
    }
}