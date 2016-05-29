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
            "start".to_string(),
            "West of the house".to_string(), 
            "You are standing in an open field west of a white house, with a boarded front door. You could circle the house to the north or south. There is a small mailbox here.\n".to_string())
        ).with(room::Exit::new(
            room::Direction::North,
            "northhouse".to_string())
    ).build();
    
    world.create_now().with(room::Room::new(
            "northhouse".to_string(),
            "North of the house".to_string(), 
            "There is a bit more of a breeze this side of the house but the warm summer sun is keeping from getting cold. A boarded enterance to the house is here.\n".to_string())
        ).with(room::Exit::new(
            room::Direction::South,
            "start".to_string())
    ).build();
    
    let mut planner = specs::Planner::<()>::new(world, 4);
    let mut running = true;
    let mut show_desc = true;
    
    let mut current_room = "start";
    
    while running {
        
        if show_desc {
            show_desc = false;
            planner.run_custom(move |arg| {
                let (rooms, exits, ents) = arg.fetch(|w| {
                    (w.read::<room::Room>(), w.read::<room::Exit>(), w.entities())
                });
                
                for (rid, room, exit) in (&ents, &rooms, &exits).iter() {
                    if current_room == room.idname {
                        println!("ID: {:?}\n{}", rid, room.name);
                        println!("{}", room.desc);
                        println!("Exit to {:?} leads to {}", exit.direction, exit.desternation);
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
            "north" => { current_room = "northhouse";
                         show_desc = true; },
            "look" => show_desc = true,
            "quit" => { running = false;
                        println!("Goodbye"); 
            },
            _ => println!("Huh?"),
        }
    }
}