use std::fs;

fn main() {
   // init
   print!("\x1B[2J\x1B[1;1H"); // put cursor at first row and col
   

   /* let board: Board = */ select_map();
   let board = initialize_test_board(); // TEMPORARY
   
   loop {
      render_board(board);
      //board = update_board(board);
      break;
   }

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
   
   for path in fs::read_dir("maps").unwrap() {
      let path = path.unwrap().path().display().to_string();
      dirs.push(path);
   }

   return dirs;
}

fn generate_map_names() -> Vec<String> {
   // takes directories of all maps and returns their names
   let dirs = generate_map_directories();

   let mut map_names: Vec<String> = Vec::new();

   for dir in dirs {
      let last_slash_index: usize = dir.rfind('/').unwrap();
      let dot_index: usize = dir.chars().count() - 4;
      let slice: String = (&dir[last_slash_index+1..dot_index]).to_string(); 
      map_names.push(slice);
   }
   
   return map_names;
}


fn select_map() /* -> Board */ {
   // 1. list names of maps
   let dir_names = generate_map_directories();
   let map_names = generate_map_names();

   let mut buffer: String = String::from("Available Maps:\n");
   for map in map_names {
      buffer.push_str(&map.to_string());
      buffer.push_str("\n");
   }
   print!("{}", buffer);

   // 2. ask user to select a map
   //let user_input = "1";

   // 3. user_input -> directory 
   //let index: usize = map_names.iter().position(|r| r == user_input).unwrap();
   //let dir = &dir_names[index];

   // 4. let matrix: vec![vec!] = load_map_from_file(dir);

   // 5. create Board from matrix and return
}

fn load_map_from_file() /* -> vec![vec!] */ {
   // print names or all available maps from generate_map_directories()
   // depending on input load chosen map
   let dirs = generate_map_directories();
}

fn initialize_test_board() -> Board {
   let board = Board {
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
