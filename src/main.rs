// for this project I heavily referenced Jude Southworth's github
// @ https://github.com/JellyWX/adventure-rs/blob/85903392d74b58889ede2a84b6dc5308f02594d6/src/main.rs
// I also referenced a code review thread on their project
// @ https://codereview.stackexchange.com/questions/205066/beginner-rust-text-adventure

// For structs and enums that you define, you’ll need to implement PartialEq to assert that values of
// those types are equal or not equal. You’ll need to implement Debug to print the values when the assertion
// fails. Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5, this is
// usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum
// definition. See Appendix C, “Derivable Traits,” for more details about these and other derivable traits.

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
// &self is a parameter for all functions
// inside an `impl` for a struct. Even
// if the function only prints there still must
// be an &self as an input parameter for the function
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
            println!("({}) {}", index, item.name);
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

    // fn that displays the instructions commands. 
    fn instruction(&self) {
        println!("\nIn command line type in:");
        println!("\t`look' to look around the current room");
        println!("\t`inspect' to inspect the items in the current room.");
        println!("\t`move <room no.>' to switch room.");
        println!("\t`inventory' to view your current inventory.");
        println!("\t`take <item no.>' to take item from current room.\n");
    }

    // displays the story of the game.
    fn story(&self) {
        println!("\n\tOur story begins with a seven year old girl named Victoria. Like every other school day
        she wakes up and gets ready for school. However, today is a very special day. Today, Victoria will
        walk to school by herself! This is a walk she has done hundred of times since kindergarten. Will she
        make it to school with not problems or is today different? Is today the day she will have to become a
        Little Warrior!\n")
    }

    // displays the title of the game. used http://www.kammerl.de/ascii/AsciiSignature.php for text to ascii art
    fn game_title(&self) {
        println!("
                    dP        dP d888888P d888888P dP         88888888b    dP   dP   dP  .d888888   888888ba   888888ba  dP  .88888.   888888ba  
                    88        88    88       88    88         88           88   88   88 d8'    88   88    `8b  88    `8b 88 d8'   `8b  88    `8b 
                    88        88    88       88    88        a88aaaa       88  .8P  .8P 88aaaaa88a a88aaaa8P' a88aaaa8P' 88 88     88 a88aaaa8P' 
                    88        88    88       88    88         88           88  d8'  d8' 88     88   88   `8b.  88   `8b. 88 88     88  88   `8b. 
                    88        88    88       88    88         88           88.d8P8.d8P  88     88   88     88  88     88 88 Y8.   .8P  88     88 
                    88888888P dP    dP       dP    88888888P  88888888P    8888' Y88'   88     88   dP     dP  dP     dP dP  `8888P'   dP     dP 
                    ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    }
}

// struct for item. only contains 'name' as a string
struct Item {
    name: String,
}

struct Enemy {
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
            name: format!("Bedroom"), // 0
            description: format!("Victoria wakes up an changes for school. She finds that her mother and father \nare not home. Being a responsible girl she knows she has to go to school.\n"),
            exits: vec![1],
            items: vec![]
        },

        Room {
            name: format!("Living Room"), //1
            description: format!("Victoria heads to the living room. The living room is clean and she sees her pet cat Stormy sleeping on the couch."),
            exits: vec![2,0],
            items: vec![ Item {
                name: format!("Backpack")
            }]
        },

        Room {
            name: format!("Outside Condo Complex"), // 2
            description: format!("Victoria is now outside her condo. Outside the weather is sunny and temperature is mild.\n She walks past the other condos heading towards school."),
            exits: vec![3,1,4],
            items: vec![]
        },

        Room {
            name: format!("Main Street"), // 3
            description: format!("Victoria is now facing the main street the condo complex is of off. The street is to busy to cross and to walk next to."),
            exits: vec![2],
            items: vec![]
        },

        // will need to implement enemies once player reaches this room. Will also need to implement hit actions/point possibly
        Room {
            name: format!("Path"), //4
            description: format!("Victoria is walking through a path in a wooden area. She sees a small creature in the middle of the path."),
            exits: vec![2,5],
            items: vec![ Item {
                name: format!("Sword")
            }]
        },

        Room {
            name: format!("Portal"), //5
            description: format!("Victoria is floating through a blue wormhole. She appears to have control of which direction to go."),
            exits: vec![4, 6],
            items: vec![]
        },

        Room {
            name: format!("Portal Cave Dark Zone"), //6
            description: format!("Victoria falls out of the portal into a dark zone of a cave. \nThe air is stale and the only light in the room is from the blue portal. Maybe there is something in the cave conveniently placed to light our path?"),
            exits: vec![5, 7, 8],
            items: vec![ Item {
                name: format!("Flashlight")
            }]
        },

        // perhaps goblin here or silent cave path
        Room {
            name: format!("Noisy cave path"), //7
            description: format!("Victoria takes the noisy cave path. She is scared and hopes the noise is nothing.\n As she points her flashlight around she notices the noise is water dripping down from the walls of the cave."),
            exits: vec![6, 9],
            items: vec![]
        },

        // perhaps gobling here or noisy cave path
        Room {
            name: format!("Silent cave path"), // 8
            description: format!("Victoria take the silent cave path. The room as very dark but there appears to be nothing in this cave room.\n Victoria then steps on something hard. She looks down and sees a skeletal remains!\n
                                  Out of nowhere a goblin comes out to attack her!"),
            exits: vec![6, 9],
            items: vec![]
        },

        Room {
            name: format!("Light at the end of the cave"), // 9
            description: format!("After a grueling trek through the cave passages Victoria finally sees daylight! While walking towards the daylight something catches her eye.\n
                                  There is something shiny on the ground."),
            exits: vec![7, 8, 10],
            items: vec![ Item {
                name: format!("Shiny Ring")
            }]
        },

        // no enemy here
        // if time implement a dead end from that the player has to return back to "outside the cave"
        Room {
            name: format!("Outside the Cave"), // 10
            description: format!("Finally! Victoria has exited the cave! She opens up her backpack and find a water bottle her dad always packs in her backpack.\n
                                  She hopes her parents are okay. She knows she has to continue to defeat the evil witch. After drinkin some water Victoria places\n
                                  the water bottle in her backpack."),
            exits: vec![9, 11],
        },

        // enemies: troll, give troll the shiny ring
        Room {
            name: format!("Bridge"), // 11
            description: format!("Victoria walks towards the bridge and starts to cross it. 'HEY! NO ONE CROSSES THIS BRIDGE WITHOUT PAYING TOLL. I have to save up\n
                                to buy a new ring since I lost my last one."),
            exits: vec![10, 12],
        },

        // figure out a way to change sword to Warrior Sword. Additionally, find a way for the wizard to open a portal
        Room {
            name: format!("Field"), // 12
            description: format!("After crossing the bridge our Little Warrior rests in the field next to a tree. The shade feels nice\n.
                                 She starts dozing off when a voice speaks to her. 'Hello little one. I am Zolo the Magical Wizard. I have\n
                                 watching you through my crystal ball and am impressed with your progress. I know you are on a quest to\n
                                 the Evil Witch,' said the Zolo. Victoria draws her sword thinking Zolo may attack her. Zolo says, 'No\n
                                 worries Little Warrior. To turn your sword into the Warrior Sword!' Zolo touches her sword and a bright\n
                                 light blinds Victoria. When the light fade her swords is bigger, shinnier, and has a golden hilt. `Wow!\n
                                 Thank you', Victoria says. With the Warrior sword in her hand Zolo opens up a portal. Victoria looks back\n
                                 at Zolo one more time and then back at the portal."),
            exits: vec![11, 13],
            items: vec![ Item {
                name: format!("Warrior Sword")
            }]
        },

        Room {
            name: format!("Portal"), // 13
            description: format!("Victoria takes the portal. She is surrounde this time by a red worm hole. She sees the field she left\n
                                  behind her and what looks to be her school in front of her. She takes a deep breath and thinks about her\n
                                  her journey so far. Picking up a sword, the cave, the goblins, the bridge troll, and Zolo the Wizard. She
                                  opens her eyes and decides it is time to end this. She then chooses her exit."),
            exits: vec![12, 14],
        },

        // attack the evil witch
        Room {
            name: format!("Victoria's school"), // 14
            description: format!("Victoria falls out the portal and lands on fer feet in front of the school. There are dark clouds all over\n
                                  the sky and in front of the school is a figure. The figure has green skin and a point nose covered in oozing
                                  warts. The greek skinned person also has a dirty tattered dress. This is the Evil Witch! Victoria draws\n
                                  the Warrior Sword. Victoria shouts, `Time to end this!`"),
            exits: vec![13, 15],
        },

        Room {
            name: format!("Victory"), // 14
            description: format!("Victoria falls out the portal and lands on fer feet in front of the school. There are dark clouds all over\n
                                  the sky and in front of the school is a figure. The figure has green skin and a point nose covered in oozing
                                  warts. The greek skinned person also has a dirty tattered dress. This is the Evil Witch! Victoria draws\n
                                  the Warrior Sword. Victoria shouts, `Time to end this!`"),
            exits: vec![13, 15],
        },

    ];

    // instansiate the player object which is of type 'Game'
    let mut player = Game {
        current_room: 0,
        rooms,
        inventory: vec![],
    };

    // will need to make this a function so it can be called to view instruction
    //println!("Type `look' to look around and 'inspect' to inspect items in room. Type `move <room no>' to switch room");
    player.game_title();
    player.story();
    player.instruction();
    player.cur_room().look();
    player.exits();


    // this for loop and the logic for using stdin is taken from
    // https://codereview.stackexchange.com/questions/205066/beginner-rust-text-adventure
    // I used the comment written by user Shepmaster @ https://codereview.stackexchange.com/users/32521/shepmaster

    // takes stdin input and splits the input by white space.
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap_or_else(|e| panic!("Error occured reading stdin: {}", e));
        let mut commands = input.trim().split_whitespace();

        //player.instruction();


        // commands records all stdin entered by the player.
        match commands.next() {
           /* Some("attack")=>{

            }*/
            
            
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
                player.cur_room().look();
                player.exits();
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
            _ => {
                println!("\nPlease enter a proper command!");
                player.instruction();
            }
        }
    }
}
