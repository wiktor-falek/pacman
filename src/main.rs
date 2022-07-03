fn main() {
   // initialization
   let board = generate_test_board();
   print_board(board);

   /*
   loop {
      // update
      
      // draw
      print_board(board);
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

/* Temporary level 
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 
1 0 1 1 1 1 0 1 1 1 0 1 1 0 1 1 1 1 0 1 
1 0 1 1 1 1 0 1 1 1 0 1 1 0 1 1 1 1 0 1 
1 0 1 0 0 0 0 0 1 1 0 1 0 0 0 0 0 1 0 1 
1 0 1 0 1 1 1 0 1 1 0 1 0 1 1 1 0 1 0 1 
1 0 1 0 1 1 1 0 1 1 0 1 0 1 1 1 0 1 0 1 
1 0 1 0 1 1 0 0 0 0 0 0 0 0 1 1 0 1 0 1 
1 0 1 0 1 1 0 1 1 0 1 1 1 0 1 1 0 1 0 1 
1 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 1 
1 0 1 0 1 1 0 1 1 1 1 1 1 0 1 1 0 1 0 1 
1 0 1 0 1 1 0 0 0 0 0 0 0 0 1 1 0 1 0 1 
1 0 1 0 1 1 1 0 1 0 1 1 0 1 1 1 0 1 0 1 
1 0 1 0 1 1 1 0 1 0 1 1 0 1 1 1 0 1 0 1 
1 0 1 0 0 0 0 0 1 0 1 1 0 0 0 0 0 1 0 1 
1 0 1 1 1 1 0 1 1 0 1 1 1 0 1 1 1 1 0 1 
1 0 1 1 1 1 0 1 1 0 1 1 1 0 1 1 1 1 0 1 
1 0 1 1 1 1 0 1 1 0 1 1 1 0 1 1 1 1 0 1
1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 
*/


fn generate_test_board() -> Board {
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
