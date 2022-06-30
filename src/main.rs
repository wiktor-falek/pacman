use std::collections::HashMap;

fn main() {
   // main game loop
   loop {
      // update
      // draw
   }
}

enum Entity {
   Empty = 0,
   Wall = 1,
   Coin = 2,
   Boost = 3, 
   Player = 4,
   Ghost = 5
}


struct Board {
   width: i32,
   height: i32,
   map: Vec<Vec<i32>>
}

fn generate_board(width: i32, height: i32) -> Board {
   let mut board = Board {
      width: width,
      height: height,
      map: vec![vec![0; width as usize]; height as usize]
   };

    // generate map
    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                board.map[y as usize][x as usize] = 1;
            }
        }
    }

   return board;
}

fn print_board(board: Board) {
   fn entity_num_as_char(num: i32) -> &str {
      let entity_to_char = HashMap::new();
      entity_to_char.insert(
         /*
         Empty = 0,
         Wall = 1,
         Coin = 2,
         Boost = 3, 
         Player = 4,
         Ghost = 5
         */
         0: ,
         1: ,
         2: ,
         3: ,
         4: ,
         5: 
      )
   }
   let mut entity_chars = HashMap::new();

   let mut buffer = String::new();
   for row in board.map {
      for val in row {
         buffer.push(val);
      }
   }
   print!("{}", buffer)
}

// board
// game loop
// player
// ghost
