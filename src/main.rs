// for this project I heavily referenced Jude Southworth's github 
// @ https://github.com/JellyWX/adventure-rs/blob/85903392d74b58889ede2a84b6dc5308f02594d6/src/main.rs
// I also referenced a code review thread on their project 
// @ https://codereview.stackexchange.com/questions/205066/beginner-rust-text-adventure

use std::io::{stdin, BufRead};

struct Game {
    cur_room: usize,
    inventory: Vec<Item>,
    rooms: Vec<Room>,
}

impl Game {
    fn room(&self) -> &Room {
        &self.rooms[self.cur_room]
    }

    fn room_mut(&mut self) -> &mut Room {
        &mut self.rooms[self.cur_room]
    }

    // display the exits for the current room to output
    fn exits(&self) {
        // print room name and number of exits
        println!("{} has {} exits:", 
        &self.room().name,
        &self.room().exits.len());

        //prints index for each exit and exit name
        for (index, exit) in self.room().exits.iter().enumerate() {
            println!("\n({}) {}", index, self.rooms[*exit].name);
        }
    }

    // displays items in inventory. logic is very similar to fn exits()
    fn view_inventory(&self) {
        println!("You have {} items: ", self.inventory.len());

        for (index, item) in self.inventory.iter().enumerate() {
            println!("\n({}) {}", index, item.name);
        }
    }

    // move to a new room
    fn move_room(&mut self, room: usize) {
        self.cur_room = self.room().exits[room];
    }

    fn take(&mut self, item: usize) -> &Item {
        let item = self.room_mut().items.remove(item);
        self.inventory.push(item);
        // returning the last item in inventory? perhaps to display?
        self.inventory.last().unwrap()
    }
}

struct Item {
    name: String,
}

struct Room {
    name: String,
    description: String,
    exits: Vec<usize>,
    items: Vec<Item>,
}

// so far the functions in this impl of Room are 
// actions that can be performed while in a room
impl Room {
    // when user types look the description of the room 
    // will print
    fn look(&self) {
        println!("{}", self.description)
    }

    fn inspect(&self) {
        println!("{} has {} items:", &self.name, &self.items.len());

        for (index, item) in self.items.iter(). enumerate() {
            println!("\n({}) {}", index, item.name);
        }
    }
}

fn main() {
    let rooms = vec![
        Room {
            name: String::from("Bedroom"),
            description: String::from("A messy little girl's room. There is a window and a door to the hallway leading to the living room."),
            exits: vec![1],
            items: vec![ 
            Item {
                name: String::from("Backpack"),
            }]
        },
    ];

    let mut player = Game {
        cur_room: 0,
        rooms,
        inventory: vec![],
    };

    println!("Type `look' to look around. Type `move <room no>' to switch room");

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap_or_else(|e| panic!("Error occured reading stdin: {}", e));
        let mut commands = input.trim().split_whitespace();

        match commands.next() {
            Some("look") => {
                player.room().look();
                player.exits();
            }

            Some("move") => {
                let args: Vec<_> = commands.take(2).collect();

                if args.len() != 1 {
                    println!("Incorrect args.");
                    continue;
                }

                let room_no: usize = match args[0].parse() {
                    Ok(a) => a,

                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                player.move_room(room_no);

                println!("You moved to {}", player.room().name);
            }

            Some("inventory") => {
                player.view_inventory();
            }

            Some("inspect") => {
                player.room().inspect();
            }

            Some("take") => {
                let args: Vec<_> = commands.take(2).collect();

                if args.len() != 1 {
                    println!("Incorrect args.");
                    continue;
                }

                let item_no: usize = match args[0].parse() {
                    Ok(a) => a,

                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                let item = player.take(item_no);

                println!("You collected {}", item.name);
            }

            _ => {}
        }
    }

}