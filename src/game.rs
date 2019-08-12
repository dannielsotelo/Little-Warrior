use crate::rooms::Room;

// struct of Game. Contains the current room the game is in, the inventory of the game, and keeps track of all the rooms in the game
#[derive(Debug, Clone)]
pub struct Game {
    pub current_room: usize,
    pub inventory: Vec<Item>,
    //gifts: Vec<Gift>
    pub rooms: Vec<Room>,
}

// implemenation for the Game Struct &self is a parameter for all functions inside an `impl` for a struct. Even
// if the function only prints there still must be an &self as an input parameter for the function
impl Game {
    // a getter that returns the current room
    pub fn cur_room(&self) -> &Room {
        &self.rooms[self.current_room]
    }

    // allows us to modify the items in a room. Used when an item is taken from a room. The room must be updated to display changes.
    pub fn room_mut(&mut self) -> &mut Room {
        &mut self.rooms[self.current_room]
    }

    // First, this fn exits() display the current room and the amount of exits in the room.
    pub fn exits(&self) {
        println!(
            "{} has {} exits:",
            &self.cur_room().name,
            &self.cur_room().exits.len()
        );

        // loops through the exits vector to display each elements index, and then the name of the element at that index. Example: (0) Bathroom
        for (index, exit) in self.cur_room().exits.iter().enumerate() {
            println!("({}) {}", index, self.rooms[*exit].name);
        }
        println!();
    }

    // Displays the inventory of the player.
    pub fn view_inventory(&self) {
        println!("You have {} items.", self.inventory.len());

        // loops through the item vector to display each elements index, and then the name of the
        // element at that index. Example: [0] Backpack
        for (index, item) in self.inventory.iter().enumerate() {
            println!("({}) {}", index, item.name);
        }
        println!();
    }

    // function the allows the player to move from one room to another room. current room updates to the
    // room we just moved to. Take a usize as a parameter. The usize is the index of the exit the player took
    pub fn move_room(&mut self, room: usize) {
        // the room in .exits[room] is the input paramter. Not referencing room in anothe struct
        self.current_room = self.cur_room().exits[room];
    }

    // allows the player to take item from a room and adds the item to the players inventory
    pub fn take(&mut self, item: usize) -> &Item {
        // item = the item removed from room
        let item = self.room_mut().items.remove(item);
        // item is added to inventory vector
        self.inventory.push(item);
        // returns the item pushed.
        self.inventory.last().unwrap()
    }

    // attack an enemy. returns a string of the enemy attacked.
    pub fn attack(&mut self, enemy: usize) -> String {
        let an_enemy: &String = &self.cur_room().enemies[enemy].name.clone();
        // println!("enemy = {:?}", an_enemy);
        self.room_mut().enemies.remove(enemy);
        an_enemy.to_string()
    }

    // fn to drop an item. item is removed from inventory vector and returned
    pub fn drop(&mut self, item: usize) -> Item {
        self.inventory.remove(item)
    }

    // fn that edits the description of a room post attack
    pub fn post_attack(&mut self) {
        if self.cur_room().name == "Path" {
            self.room_mut().description = format!("\t\tVictoria has cleared path after defeating the goblin. She can now safely approach the blue portal.")
        }
        if self.cur_room().name == "Silent cave path" {
            self.room_mut().description = format!("\t\tVictoria moves quickly to attack the goblin. The goblin doges Victoria's attack. The Goblin raise its
                hand to attack Victoria. She quickly ducks and the Goblin misses. Victoria again swipes her sword at
                the Goblin. Critical Hit! The Goblin is defeated. The is section of the cave is now clear.")
        }
        if self.cur_room().name == "Victoria's School" {
            self.room_mut().description = format!("\t\tVictoria moves to attack the Evil Witch. The Witch shoots fireballs out of her hands!
                Victoria dodges the fireballs and swings the Warrior Sword at the Witch. The Witch gets hit but stands back up and runs into the school.")
        }
        if self.cur_room().name == "Inside the School" {
            self.room_mut().description = format! {
                "\t\tThe Little Warrior stumbles on her feet. She does not know if she as any strength left but, she knows she must defeat the Evil Witch."
            }
        }
        if self.cur_room().name == "Gymnasium" {
            self.room_mut().description = format! {"\t\t'AHHHHHHHH!!!!' A shriek is heard for miles around. The Evil Witch and Little Warrior attacked and ran past each other. They turn
                around to look at each other eyes. The Evil Witch then falls. Victoria watches as the Witch's body turns into dust and all is left is her 
                black tattered dress. The Evil Witch is defeated."
            }
        }
    }

    // fn that edits the description of a room post item drop
    pub fn post_drop(&mut self) {
        if self.cur_room().name == "Bridge" {
            self.room_mut().description = format!("\t\tThe Troll, happy with a shiny ring, lets Victoria pass and heads into the cave.");
            self.room_mut().enemies.clear();
        }
    }

    // fn that displays the instructions commands.
    pub fn instruction(&self) {
        println!("\tIn the command line type in:");
        println!("\t`look' to look around the current room");
        println!(
            "\t`inspect' to inspect the items in the current room and enemies in the current room."
        );
        println!("\t`move <room no.>' to switch room.");
        println!("\t`inventory' to view your current inventory.");
        println!("\t`take <item no.>' to take item.");
        println!("\t`attack <enemey no.>` to attack enemy.");
        println!("\t'drop <item no.>' to drop item. PROTIP: DO NOT DROP ANY WEAPONS OR THEY WILL BE LOST FOREVER!!!!");
    }

    // displays the story of the game.
    pub fn story(&self) {
        println!("\tOur story begins with a seven year old girl named Victoria. Like every other school day
        she wakes up and gets ready for school. However, today is a very special day. Today, Victoria will
        walk to school by herself! This is a walk she has done hundred of times since kindergarten. Will she
        make it to school with not problems or is today different? Is today the day she will have to become a
        Little Warrior!\n")
    }

    // displays the title of the game. used http://www.kammerl.de/ascii/AsciiSignature.php for text to ascii art
    pub fn game_title(&self) {
        println!("
                    dP        dP d888888P d888888P dP         88888888b    dP   dP   dP  .d888888   888888ba   888888ba  dP  .88888.   888888ba  
                    88        88    88       88    88         88           88   88   88 d8'    88   88    `8b  88    `8b 88 d8'   `8b  88    `8b 
                    88        88    88       88    88        a88aaaa       88  .8P  .8P 88aaaaa88a a88aaaa8P' a88aaaa8P' 88 88     88 a88aaaa8P' 
                    88        88    88       88    88         88           88  d8'  d8' 88     88   88   `8b.  88   `8b. 88 88     88  88   `8b. 
                    88        88    88       88    88         88           88.d8P8.d8P  88     88   88     88  88     88 88 Y8.   .8P  88     88 
                    88888888P dP    dP       dP    88888888P  88888888P    8888' Y88'   88     88   dP     dP  dP     dP dP  `8888P'   dP     dP 
                    ooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo\n");
    }
}

// struct for item. only contains 'name' as a string
#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
}

// struct for Enemy. Only contains 'name' as a string
#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
}
