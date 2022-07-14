use std::fs;

fn main() {
   // initialization
   // let map = fs::read_to_string("maps/1.txt")
   //          .expect("Failed to read file");
   let map = load_map_from_file();

   let mut board = initialize_test_board();
   render_board(board);

   /*
   loop {
      board = update_board(board);

      // draw
      render_board(board);
   }
   */
}

enum Tile {
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

fn generate_map_directories() -> Vec<String> {
   let mut dirs = Vec::new();

   // XDDD
   for path in fs::read_dir("../../src/maps").unwrap() {
      let path = path.unwrap().path().display().to_string();
      println!("{}", path);
      dirs.push(path);
   }

   dirs
}


// str immutable
// String when you need to own or modify your string data
fn load_map_from_file() /* -> vec![vec!] */ {
   // print names or all available maps from generate_map_directories()
   // depending on input load chosen map
   let dirs = generate_map_directories();

   for d in dirs {
      println!("{}", d);
   }

}

// BOARD 

fn initialize_test_board() -> Board {
   let mut board = Board {
      width: 20,
      height: 20,
      map: vec![
         vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ],
         vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
         vec![1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1 ],
         vec![1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1 ],
         vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1 ],
         vec![1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1 ],
         vec![1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1 ],
         vec![1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1 ],
         vec![1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1 ],
         vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 ],
         vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1 ]
      ]
   };

   return board;
}

fn initialize_board(map: Vec<Vec<i32>>) -> Board {
   let height: usize = map.len();
   let width: usize = map[0].len();

   let mut board = Board {
      width: width as i32,
      height: height as i32,
      // map: load_map_from_file()
      map: vec![vec![0; width]; height]
   };

    // generate map
    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                board.map[y][x] = 1;
            }
        }
    }

   return board;
}

fn render_board(board: Board) {
   fn tile_to_char(num: i32) -> char {
      match num {
         0 => return ' ',
         1 => return '#',
         2 => return '$',
         3 => return '*',
         4 => return '@',
         5 => return '&',
         _ => panic!("{}", format!("Invalid character '{}' found in board", num))
      }
   }

   let mut buffer = String::new();
   for row in board.map {
      for val in row {
         buffer.push(tile_to_char(val));
         buffer.push_str(" ");
      }
      buffer.push_str("\n");
   }
   print!("{}", buffer);
}

// USER INPUT HANDLING