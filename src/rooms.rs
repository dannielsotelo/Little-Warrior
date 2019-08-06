use crate::game::Item;
use crate::game::Enemy;

// struct of Room. Contains the name of the room as a String, the description of the room as a String, a vector of exits, and a vector of items
#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: Vec<usize>,
    pub items: Vec<Item>,
    pub enemies: Vec<Enemy>,
    //gifts: Vec<Gift>,
}

// implemenation of the Room struct
impl Room {
    // prints the description of the current room the player is in
    pub fn look(&self) {
        println!("\n{}", self.description)
    }

    // this function executes when the player enters the 'inspect' command. This function prints out the current room name and the number of items in the room.
    pub fn inspect(&self) {
        println!("{} has {} items.", &self.name, &self.items.len());
        for (index, item) in self.items.iter().enumerate() {
            println!("({}) {}", index, item.name);
        }
        println!();
        println!("{} has {} enemies.", &self.name, &self.enemies.len());
        for (index, enemy) in self.enemies.iter().enumerate() {
            println!("({}) {}", index, enemy.name);
        }

        println!();
    }
}
