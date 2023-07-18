use rand::Rng;
use rust_embed::RustEmbed;
pub struct Book {
    book: Vec<String>,
    opening: String,
    copy: Vec<String>,
}
impl Book {
    pub fn get_next_move(&mut self, game: &str) -> &str {
        if self.book.is_empty() || game.len() > 143 {
            return "";
        }
        let mut max = self.book.len();
        let mut i = 0;
        while i < max {
            if !self.book.get(i).unwrap().starts_with(game) {
                self.book.remove(i);
                max -= 1;
            } else {
                i += 1;
            }
        }
        if self.book.is_empty() {
            return "";
        }
        let mut rng = rand::thread_rng();
        let moves = self.book.get(rng.gen_range(0..self.book.len())).unwrap();
        if game == "" {
            return &moves[game.len()..(game.len() + 5)].trim();
        }
        return &moves[game.len()..(game.len() + 6)].trim();
    }
    pub fn reset(&mut self) {
        self.book = self.copy.clone()
    }
    pub fn check(&mut self, line: &str) -> Option<String> {
        if line.starts_with(&self.opening) {
            self.opening = String::from(line);
            return Some(self.get_next_move(line).to_string());
        };

        return None;
    }
}
#[derive(RustEmbed)]
#[folder = "src/files/"]
#[prefix = ""]
struct Asset;
pub fn init_book_full() -> Book {
    let file = Asset::get("Book.txt").unwrap();
    return Book {
        book: std::str::from_utf8(file.data.as_ref())
            .unwrap()
            .split('\n')
            .map(str::to_string)
            .collect::<Vec<String>>(),
        copy: std::str::from_utf8(file.data.as_ref())
            .unwrap()
            .split('\n')
            .map(str::to_string)
            .collect::<Vec<String>>(),
        opening: "".to_string(),
    };
}
