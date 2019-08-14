# Danniel Sotelo

# Little Warrior

### How to download and run game
`git clone https://github.com/dannielsotelo/Little-Warrior.git`  
`cd Little-Warrior`  
`cargo run`

## Project Description
  
  This project for CS410 Rust will be a text based game. The main purpose of this game is for me, the developer,  
  to become more familiar with the Rust Programming Language. The story of the game is based off the many little  
  stories I tell my daughter when I walk her to school. The main character in the story is a seven year old girl.  
  Like every other school day she wakes up to go to school but, this day will be a little different.  
  When starting up the game the player will be presented with a menu of options to perform and a  
  description of their surroundings. The player will then try to go to school.

  The player of the game will travle through Rooms. While moving between rooms the player will be able to interact
  with items, enemies, and exits. Sometimes the player will pick up items to attack enemies. Other times the player
  will attack enemies or get stuck in a dead end.

#### Referenced

  for this project I referenced Jude Southworth's github  
  @ https://github.com/JellyWX/adventure-rs/blob/85903392d74b58889ede2a84b6dc5308f02594d6/src/main.rs  
  I also referenced a code review thread on their project   
  @ https://codereview.stackexchange.com/questions/205066/beginner-rust-text-adventure 

  I referenced their github and the forum codereview to view how they organized their objects and code. When
  I initially played their text-based game their game only hade 2 or 3 rooms and no enemies.


## Example Image of Little Warrior
![Example](images/Example.png)

### What in game needs to be improved
  The first would like to improve how the ```rooms``` are implemented. Currently, in ```main.rs```  
  I create a vector of  ```rooms``` and I hate it. I kept on adding features to my game and never  
  went back to find an easier or better looking implementation for all the ```rooms``` in the game.  
  I want to improve this game because I want my daughter, and soon to be second daughter, to play  
  this game. When I was 3/4 complete with the game I found a ```crate``` that keeps track of rooms,  
  enemies, and even items but, it was written in ```JSON```. Now that I have more time I want to  
  fully implement that ```crate```. I also want to implement graphics in some way. I think there  
  are crates to do this. After graphich as I would want to implement sound. This way the game would  
  be playable and keep my daughters' attention because right now, the game is pretty much a choose  
  your own adventure game. 

## License Information

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
![License](LICENSE)
