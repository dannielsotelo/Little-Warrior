// for this project I heavily referenced Jude Southworth's github
// @ https://github.com/JellyWX/adventure-rs/blob/85903392d74b58889ede2a84b6dc5308f02594d6/src/main.rs
// I also referenced a code review thread on their project
// @ https://codereview.stackexchange.com/questions/205066/beginner-rust-text-adventure

// For structs and enums that you define, you’ll need to implement PartialEq to assert that values of
// those types are equal or not equal. You’ll need to implement Debug to print the values when the assertion
// fails. Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5, this is
// usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum
// definition. See Appendix C, “Derivable Traits,” for more details about these and other derivable traits.

use little_warrior::rooms::*;
use little_warrior::game::*;
//use little_warrior::rooms::Room;
//use little_warrior::game::Item;
//use little_warrior::game::Enemy;
//use little_warrior::game::Game;
use std::io::stdin;
use std::io::BufRead;


fn main() {
    // creates a vector of rooms to be used in the game
    let rooms = vec![
        Room {
            name: format!("Bedroom"), // 0
            description: format!("\t\tVictoria wakes up an changes for school. She finds that her mother and father are not home. 
                Being a responsible girl she knows she has to go to school. She puts on her backpack and prepares to head out."),
            exits: vec![1],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        Room {
            name: format!("Living Room"), //1
            description: format!("\t\tVictoria heads to the living room. In the living room her cat, Stormy, is sleeping on the couch."),
            exits: vec![0,2],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        Room {
            name: format!("Outside Condo Complex"), // 2
            description: format!("\t\tVictoria is now outside her condo. Outside the weather is sunny and temperature is mild.
                She walks past the other condos heading towards school."),
            exits: vec![1,3,4],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        Room {
            name: format!("Main Street"), // 3
            description: format!("\t\tVictoria is now facing the main street the condo complex is of off. The street is to busy to cross and to walk next to."),
            exits: vec![2],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        // will need to implement enemies once player reaches this room. Will also need to implement hit actions/point possibly
        Room {
            name: format!("Path"), //4
            description: format!("\t\tVictoria is walking through a path in a wooden area. She sees a small creature in the middle of the path.
                There appears to be a blue portal behind the creature."),
            exits: vec![2,5],
            items: vec![ Item {
                name: format!("Sword")
            }],
            enemies: vec![ Enemy{
                name: format!("Goblin")
            }],
            //gifts: vec![]
        },

        Room {
            name: format!("Portal"), //5
            description: format!("\t\tVictoria is floating through a blue wormhole. She appears to have control of which direction to go."),
            exits: vec![4, 6],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        Room {
            name: format!("Portal Cave Dark Zone"), //6
            description: format!("\t\tVictoria falls out of the portal into a dark zone of a cave. The air is stale and the only light in the room
                is from the blue portal. Maybe there is something in the cave conveniently placed to light our path?"),
            exits: vec![5, 7, 8],
            items: vec![ Item {
                name: format!("Flashlight")
            }],
            enemies: vec![],
            //gifts: vec![],
        },

        Room {
            name: format!("Noisy cave path"), //7
            description: format!("\t\tVictoria takes the noisy cave path. She is scared and hopes the noise is nothing. 
                As she points her flashlight around she notices the noise is water dripping down from the walls of the cave."),
            exits: vec![6, 9],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        Room {
            name: format!("Silent cave path"), // 8
            description: format!("\t\tVictoria take the silent cave path. The room as very dark but there appears to be nothing in this cave room.
                Victoria then steps on something hard. She looks down and sees a skeletal remains! Out of nowhere a goblin comes out to attack her!"),
            exits: vec![6, 9],
            items: vec![],
            enemies: vec![ Enemy {
                name: format!("Goblin")
            }],
            //gifts: vec![],
        },

        Room {
            name: format!("Light at the end of the cave"), // 9
            description: format!("\t\tAfter a grueling trek through the cave passages Victoria finally sees daylight!
                While walking towards the daylight something catches her eye. There is something shiny on the ground."),
            exits: vec![7, 8, 10],
            items: vec![ Item {
                name: format!("Shiny Ring")
            }],
            enemies: vec![],
           // gifts: vec![],
        },

        // if time implement a dead end from that the player has to return back to "outside the cave"
        Room {
            name: format!("Outside the Cave"), // 10
            description: format!("\t\tFinally! Victoria has exited the cave! There is a small river outside the cave. She is very thirsty
                and drinks some water from the river. She then sits down outside the cave and rests for a moment. While she rests she 
                begins to think about her day so far. She hopes her parents are okay. She knows she has to continue to defeat the 
                evil witch. After a little break she gets up."),
            exits: vec![9, 11],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        // enemies: troll, give troll the shiny ring
        Room {
            name: format!("Bridge"), // 11
            description: format!("\t\tVictoria walks towards the bridge and starts to cross it. 'HEY! NO ONE CROSSES THIS BRIDGE WITHOUT PAYING TOLL.
                I have to save up to buy a new ring since I lost my last one.' Says a Troll with purple eyes, green skin, and red claws.
                Victoria remembers she has a ring. NOTE: Please only drop the ring!!!"),
            exits: vec![10, 12],
            items: vec![],
            enemies: vec![ Enemy {
                name: format!("Troll")
            }],
            //gifts: vec![],
        },

        // figure out a way to change sword to Warrior Sword. Additionally, find a way for the wizard to open a portal
        Room {
            name: format!("Field"), // 12
            description: format!("\t\tAfter crossing the bridge our Little Warrior rests in the field next to a tree. The shade feels nice.
                She starts dozing off when a voice speaks to her. 'Hello little one. I am Zolo the Magical Wizard. I was
                watching you through my crystal ball and I am impressed with your progress. The reason you have had a strang day is because
                of the Evil Witch,' said Zolo. Victoria draws her sword thinking Zolo may attack her. Zolo says, 'No
                worries Little Warrior. I am here to give you the Warrior Sword!' Zolo claps his hands and a bright
                light blinds Victoria. When the light fades a shiny sword with a golden hilt appears on the ground. `Wow!
                Thank you', Victoria says. Zolo opens up a portal and say, 'This way leads to your destiny and do not forget
                to take the Warrior Sword off the ground!'"),
            exits: vec![11, 13],
            items: vec![ Item {
                name: format!("Warrior Sword")
            }],
            enemies: vec![],
            //gifts: vec![],
        },

        Room {
            name: format!("Portal"), // 13
            description: format!("\t\tVictoria takes the portal. She is surrounded this time by a red worm hole. She sees the field she left
                behind her and what looks lik her school in front of her. She takes a deep breath and thinks about her
                her journey so far. Picking up a sword, the cave, the goblins, the bridge troll, and Zolo the Wizard. She
                opens her eyes and decides it is time to end this. She then chooses her exit."),
            exits: vec![12, 14],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },

        // attack the evil witch
        Room {
            name: format!("Victoria's School"), // 14
            description: format!("\t\tVictoria falls out the portal and lands on fer feet in front of the school. There are dark clouds all over
                the sky and in front of the school is a figure. The figure has green skin and a pointy nose covered in oozing
                warts. The green skinned figure also has a dirty tattered dress. This is the Evil Witch! Victoria draws
                the Warrior Sword. Victoria shouts, `Time to end this!`"),
            exits: vec![13,15],
            items: vec![],
            enemies: vec![ Enemy{
                name: format!("The Evil Witch")
            }],
            //gifts: vec![],
        },

        // find out what to do once witch is defeated. 
        Room {
            name: format!("Inside the School"), // 15
            description: format!("\t\tVictoria runs inside the school after the Evil Witch. Victoria is exhausted and her left shoulder
                is sore when she hit the ground dodging the Evil Witch's fireballs. The Evil Witch turns around and shoots a lightning bolt
                from her hands. Victoria is hit and falls down in pain! The Evil Witch yells, 'I have you know you insolute little girl!'
                Victoria flips back up and shouts, 'not today!', and Victoria charges the Evil Witch. The Little Warrior dodges two thunderbolts
                and hits the Evil Witch. The Witch screams in pain and runs into the gymnasium."),
            exits: vec![14, 16],
            items: vec![],
            enemies: vec![Enemy{
                name: format!("The Evil Witch")
            }],
            //gifts: vec![],
        },

        Room {
            name: format!("Gymnasium"), // 16
            description: format!("\t\tNow inside the Gymnasium our Little Warrior prepares herself to deliver the final blow. There appears
                to be an Orange Portal in the room and she sees her bedroom in the portal. 'After I destory you I will go to your world and
                destory it,' screams the Witch. The Evil Witch then screams and the walls of the gym fall down. A storm engulfs the two combatants. 
                The Evil Witch shrieks, 'I WILL KILL YOU!' But, the Little Warrior is not afraid. The Little Warrior and Evil Witch charge each other!"),
            exits: vec![15, 17],
            items: vec![],
            enemies: vec![Enemy{
                name: format!("The Evil Witch")
            }],
            //gifts: vec![],
        },

        Room {
            name: format!("Bedroom thru Orange Portal"), // 17
            description: format!("\t\tWakes up in a sweat in he bed. She looks around and sees her cat Stormy sleeping by her feet and her Nintendo
                Switch still turned on paused on her newest Zelda game. Her mother and father come into the room. 'Are you okay my Little Warrior?' 
                says her dad. Victoria says, 'yes, I had the strangest dream.' Her mother says, 'it was only a dream. Now go to sleep. You have school
                in the morning.' Her parents kiss her good night and leave her bedroom. Victoria rests her head on her pillow but looks outside her
                window one last time before bed. She sees Zolo wave at her and then jumps into a Blue Portal. In shock Victoria yells, 'DAD! I NEED
                TO TELL YOU SOMETHING!'"),
            exits: vec![],
            items: vec![],
            enemies: vec![],
            //gifts: vec![],
        },
    ];

    // instansiate the player object which is of type 'Game'
    let mut player = Game {
        current_room: 0,
        rooms,
        inventory: vec![],
    };

    // game title is displayed
    player.game_title();
    // story is displayed
    player.story();
    //instructions is displayed
    player.instruction();

    //    player.cur_room().look();

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
                println!();
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

                if !player.cur_room().enemies.is_empty() {
                    println!("There is an enemy in your path. You must take care of the enemy before proceeding.\n");
                    continue;
                }

                // player is moved to new room with move_room() fn
                player.move_room(room_no);

                // displays player moved to new room
                println!("\nYou moved to {}", player.cur_room().name);
                player.instruction();
                player.cur_room().look();
                println!();
                player.exits();
            }

            // when player enters `inventory` view_inventory is executed and the players current inventory is displayed
            Some("inventory") => {
                player.view_inventory();
            }

            // when player enters 'inspect' the inspect() function is executed on the current room and the items in the room are displayed
            Some("inspect") => {
                player.cur_room().inspect();
            }

            // when player enters 'take' an args vector is created. The vector takes the last 2 strings entered by the user.
            // Example: `take sword`
            Some("take") => {
                if player.cur_room().items.is_empty() {
                    player.cur_room().inspect();
                    println!("There is nothing to take!\n");
                    continue;
                }
                let args: Vec<_> = commands.take(2).collect();

                // if args contains anything but 2 commands then an error message is displayed
                if args.len() != 1 {
                    println!("Incorrect command.");
                    continue;
                }
                // gets the item_no from args. If item number entered by player is not one of the item numbers allowed the program
                // prints standard error message and terminates
                let item_no: usize = match args[0].parse() {
                    Ok(a) => a,

                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                // calls take() on player and adds items to players inventory. variable item stores the name of item taken
                let item = player.take(item_no);
                println!("You collected a {}\n", item.name);
            }

            // when attack command is enter player will start attacking enemy if certain conditions are met
            Some("attack") => {
                // if the players inventory is empty then they cannot attack
                if player.inventory.is_empty() {
                    println!("You have nothing to attack with!");
                    continue;
                }

                // if there are not enemies in the room then player cannot attack
                if player.cur_room().enemies.is_empty() {
                    player.cur_room().inspect();
                    println!("There are no enemies to attack. ");
                    continue;
                }

                // collect 2 arguments for input
                let args: Vec<_> = commands.take(2).collect();

                // if there are not 2 args then print error statement
                if args.len() != 1 {
                    println!("Incorrect command.");
                    continue;
                }

                // get enemy number
                let enemy_no: usize = match args[0].parse() {
                    Ok(a) => a,

                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                // attack enemy by their enemy number
                let enemy = player.attack(enemy_no);
                println!("You attacked the {}\n", enemy);

                // this will change description of room after enemy is attacked
                player.post_attack();
            }

            Some("drop") => {
                // if the players inventory is empty then they cannot give anything
                if player.inventory.is_empty() {
                    println!("You have nothing to give!");
                    continue;
                }

                // collect 2 arguments for input
                let args: Vec<_> = commands.take(2).collect();

                // if there are not 2 args then print error statement
                if args.len() != 1 {
                    println!("Incorrect command.");
                    continue;
                }

                // get enemy number
                let item_no: usize = match args[0].parse() {
                    Ok(a) => a,

                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                // calls take() on player and adds items to players inventory. variable item stores the name of item taken
                let item = player.drop(item_no);
                println!("You dropped a {}\n", item.name);
                player.post_drop();
            }

            // any other case
            _ => {
                println!("Please enter a proper command!\n");
                player.instruction();
            }
        }
    }
}

