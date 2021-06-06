struct Snake {
}

struct Apple {

}

struct Game {
    height: u8,
    width: u8,
}

impl Game {
    fn new(height: u8, width: u8) -> Game{
        Game {
            height,
            width,
        }
    }

    fn render(&self) {
        println!("Height: {}", self.height);
        println!("Width: {}", self.width);
    }
}

fn main() {
    let game = Game::new(2,4);
    game.render();
}