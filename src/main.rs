extern crate specs;

mod room;

fn main() {
    
    let mut world = specs::World::new();
    world.register::<room::Room>();
    world.register::<room::Exits>();
    world.create_now().with(room::Room::new("start".to_string(),"West of the house".to_string(), "You are standing in an open field west of a white house, with a boarded front door. You could circle the house to the north or south. There is a small mailbox here.".to_string())).build();
    
    let mut running = true;
    
    /*let (entities, rooms) = arg.fetch(|w| {
        (w.entities(), w.read::<room::Room>())
    });*/
    
    //println!("{}", start.get_id());
    //println!("{}", start.desc);
    println!("What now ?:");
}
