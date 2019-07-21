use std::io::{stdin, BufRead};

struct Game {
    room: usize,
    inventory: Vec<Item>,
    rooms: Vec<Room>,
}

impl Game {
    fn room(&self) -> &Room {
        &self.rooms[self.room]
    }

    fn room_mut(&mut self) -> &mut Room {
        &mut self.rooms[self.room]
    }

    fn exits(&self) {
        println!(
            "{} has {} exits:",
            &self.room().name,
            &self.room().exits.len()
        );

        for (index, exit) in self.room().exits.iter().enumerate() {
            println!("({}) {}", index, self.rooms[*exit].name);
        }
    }

    fn view_inventory(&self) {
        println!("You have {} items:", self.inventory.len());

        for (index, item) in self.inventory.iter().enumerate() {
            println!("\n({}) {}", index, item.name);
        }
    }

    fn move_room(&mut self, room: usize) {
        self.room = self.room().exits[room];
    }

    fn take(&mut self, item: usize) -> &Item {
        let item = self.room_mut().items.remove(item);
        self.inventory.push(item);
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

impl Room {
    fn look(&self) {
        println!("{}", self.description)
    }

    fn inspect(&self) {
        println!("{} has {} items:", &self.name, &self.items.len());

        for (index, item) in self.items.iter().enumerate() {
            println!("\n({}) {}", index, item.name);
        }
    }
}

fn main() {
    let rooms = vec![
        Room {
            name: String::from("Bedroom"),
            description: String::from("A tidy, clean bedroom with 1 door and a balcony"),
            exits: vec![1, 2],
            items: vec![ Item {
                name: String::from("Key"),
            }]
        },

        Room {
            name: String::from("Balcony"),
            description: String::from("An outdoor balcony that overlooks a gray garden"),
            exits: vec![3],
            items: vec![]
        },

        Room {
            name: String::from("Landing"),
            description: String::from("A carpetted landing with doors leading off it. It overlooks a large living space. A set of stairs leads down"),
            exits: vec![1],
            items: vec![]
        },

        Room {
            name: String::from("Doghouse"),
            description: String::from("a dog house"),
            exits: vec![1],
            items: vec![]
        },

        Room {
            name: String::from("Bathroom"),
            description: String::from("a bathroom with a toilet"),
            exits: vec![0],
            items: vec![]
        },
    ];

    let mut player = Game {
        room: 0,
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