// for this project I heavily referenced Jude Southworth's github 
// @ https://github.com/JellyWX/adventure-rs/blob/85903392d74b58889ede2a84b6dc5308f02594d6/src/main.rs
// I also referenced a code review thread on their project 
// @ https://codereview.stackexchange.com/questions/205066/beginner-rust-text-adventure

// allows program to read standard input and utilized BufRead 
// to be able to read line by line
use std::io::{stdin, BufRead};

// struct of Game. Contains the current room the game is in, 
// the inventory of the game, and keeps track of all the rooms
// in the game
struct Game {
    current_room: usize,
    inventory: Vec<Item>,
    rooms: Vec<Room>,
}

// implemenation for the Game Struct
impl Game {
    // a getter that returns the current room
    fn cur_room(&self) -> &Room {
        &self.rooms[self.current_room]
    }

    // allows us to modify the items in a room.
    // used when an item is taken from a room.
    // the room must be updated to display changes.
    fn room_mut(&mut self) -> &mut Room {
        &mut self.rooms[self.current_room]
    }

    //  First, this fn exits() display the current room
    //  and the amount of exits in the room.
    fn exits(&self) {
        println!(
            "{} has {} exits:",
            &self.cur_room().name,
            &self.cur_room().exits.len()
        );

        // loops through the exits vector to display
        // each elements index, and then the name of 
        // the element at that index.
        // Example: [0] Bathroom
        for (index, exit) in self.cur_room().exits.iter().enumerate() {
            println!("({}) {}", index, self.rooms[*exit].name);
        }
    }

    // Displays the inventory of the player.
    fn view_inventory(&self) {
        println!("You have {} items:", self.inventory.len());

        // loops through the item vector to display
        // each elements index, and then the name of the 
        // element at that index.
        // Example: [0] Backpack
        for (index, item) in self.inventory.iter().enumerate() {
            println!("\n({}) {}", index, item.name);
        }
    }

    // function the allows the player to move from one
    // room to another room. current room updates to the
    // room we just moved to. Take a usize as a parameter.
    // The usize is the index of the exit the player took
    fn move_room(&mut self, room: usize) {
        // the room in .exits[room] is the input paramter. Not referencing room in anothe struct
        self.current_room = self.cur_room().exits[room];
    }

    // allows the player to take item from a room
    // and adds the item to the players inventory
    fn take(&mut self, item: usize) -> &Item {
        // item = the item removed from room
        let item = self.room_mut().items.remove(item);
        // item is added to inventory vector
        self.inventory.push(item);
        // returns the item pushed.
        self.inventory.last().unwrap()
    }
}

// struct for item. only contains 'name' as a string
struct Item {
    name: String,
}

// struct of Room. Contains the name of the room as a String,
// the description of the room as a String, a vector of exits,
// and a vector of items
struct Room {
    name: String,
    description: String,
    exits: Vec<usize>,
    items: Vec<Item>,
}

// implemenation of the Room struct
impl Room {
    // prints the description of the current room the player is in
    fn look(&self) {
        println!("{}", self.description)
    }

    // this function executes when the player enters the 'inspect'
    // command. This function prints out the current room name
    // and the number of items in the room.
    fn inspect(&self) {
        println!("{} has {} items:", &self.name, &self.items.len());

        // loops through the item vector in the current room
        // and displays the items index and name
        for (index, item) in self.items.iter().enumerate() {
            println!("\n({}) {}", index, item.name);
        }
    }
}

fn main() {
    // creates a vector of rooms to be used in the game
    // TODO: change name of rooms to something unique. too many rooms in this program
    let rooms = vec![
        Room {
            name: String::from("Bedroom"),
            description: String::from("A tidy, clean bedroom with 1 door and a balcony"),
            exits: vec![3, 4],
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

    // instansiate the player object which is of type 'Game'
    let mut player = Game {
        current_room: 0,
        rooms,
        inventory: vec![],
    };

    println!("Type `look' to look around and 'inspect' to inspect items in room. Type `move <room no>' to switch room");

    // this for loop and the logic for using stdin is taken from 
    // https://codereview.stackexchange.com/questions/205066/beginner-rust-text-adventure
    // I used the comment written by user Shepmaster @ https://codereview.stackexchange.com/users/32521/shepmaster
    
    // takes stdin input and splits the input by white space. 
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap_or_else(|e| panic!("Error occured reading stdin: {}", e));
        let mut commands = input.trim().split_whitespace();

        // commands records all stdin entered by the player.
        match commands.next() {
            // when player enters `look` the current room is 
            // retrieved and the .look() is executed on that current room
            // the room description is then displayed
            Some("look") => {
                player.cur_room().look();
                player.exits();
            }

            // the 'move' is entered
            Some("move") => {
                // args variale created as a vector. Takes in 2 paramters
                // example: `move` is the command and `0` is the exit taken
                let args: Vec<_> = commands.take(2).collect();

                // error if there is more than 2 input arguments for args
                if args.len() != 1 {
                    println!("Incorrect args.");
                    continue;
                }

                // gets the room_no from args. If exit number entered by
                // player is not one of the exits allowed the program
                // prints standard error message and terminates
                let room_no: usize = match args[0].parse() {
                    Ok(a) => a,

                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                // player is moved to new room with move_room() fn
                player.move_room(room_no);

                // displays player moved to new room
                println!("You moved to {}", player.cur_room().name);
            }

            // when player enters `inventory` view_inventory is executed
            // and the players current inventory is displayed
            Some("inventory") => {
                player.view_inventory();
            }

            // when player enters 'inspect' the inspect() function is executed
            // on the current room and the items in the room are displayed
            Some("inspect") => {
                player.cur_room().inspect();
            }

            // when player enters 'take' an args vector is created. The vector
            // takes the last 2 strings entered by the user.
            // Example: `take sword`
            Some("take") => {
                let args: Vec<_> = commands.take(2).collect();

                // if args contains anything but 2 commands
                // then an error message is displayed
                if args.len() != 1 {
                    println!("Incorrect args.");
                    continue;
                }
                // gets the item_no from args. If item number entered by
                // player is not one of the item numbers allowed the program
                // prints standard error message and terminates
                let item_no: usize = match args[0].parse() {
                    Ok(a) => a,

                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                // calls take() on player and adds items
                // to players inventory
                let item = player.take(item_no);

                println!("You collected {}", item.name);
            }
            // any other case
            _ => {}
        }
    }
}