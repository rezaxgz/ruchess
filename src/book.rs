// use rand::Rng;
// use std::fs;
// pub struct Sorted {
//     vec: Vec<String>,
// }
// impl Sorted {
//     pub fn find(&mut self, x: String) -> isize {
//         let mut r = self.vec.len() - 1;
//         let mut l = 0;
//         while l <= r {
//             let m = l + (r - l) / 2;

//             // Check if x is present at mid
//             if self.vec.get(m).unwrap().starts_with(&x) {
//                 return m as isize;
//             }

//             // If x greater, ignore left half
//             if self.vec.get(m).unwrap() < &x {
//                 l = m + 1;
//             }
//             // If x is smaller, ignore right half
//             else {
//                 r = m - 1;
//             }
//         }
//         return -1;
//     }
//     pub fn get(&self, i: usize) -> Option<&String> {
//         return self.vec.get(i);
//     }
//     pub fn filter(&mut self, x: String) {
//         if self.vec.is_empty() {
//             return;
//         }
//         let mut max = self.vec.len();
//         let mut i = 0;
//         while i < max {
//             if !self.vec.get(i).unwrap().starts_with(&x) {
//                 self.vec.remove(i);
//                 max -= 1;
//             }
//             i += 1;
//         }
//     }
//     pub fn len(&self) -> usize {
//         return self.vec.len();
//     }
//     pub fn is_empty(&self) -> bool {
//         return self.vec.is_empty();
//     }
// }
// pub fn init_sorted(book: String) -> Sorted {
//     let mut vec: Vec<String> = book.split('\n').map(str::to_string).collect();
//     vec.sort();
//     return Sorted { vec };
// }
// pub struct Book {
//     book: Sorted,
//     opening: String,
// }
// impl Book {
//     pub fn get_next_move(&mut self, game: &String) -> &str {
//         self.book.filter(game.clone());
//         if !self.book.is_empty() {
//             let mut rng = rand::thread_rng();
//             let moves = self.book.get(rng.gen_range(0..self.book.len())).unwrap();
//             if game == "" {
//                 return &moves[game.len()..(game.len() + 5)].trim();
//             } else {
//                 return &moves[game.len()..(game.len() + 6)].trim();
//             }
//         } else {
//             return "";
//         }
//     }
//     pub fn get_opening(&self) -> String {
//         return self.opening.clone();
//     }
// }
// pub fn init_book(book: String, opening: String) -> Book {
//     return Book {
//         book: init_sorted(book),
//         opening,
//     };
// }
// pub fn init_book_full() -> Book {
//     return Book {
//         book: init_sorted(
//             fs::read_to_string("src/files/Book.txt")
//                 .expect("Should have been able to read the file"),
//         ),
//         opening: "".to_string(),
//     };
// }
