use adversarial_2048::{ai::Ai, human::Human, random::Random, Game};

fn main() {
    let mut g = Game::new(Box::new(Random), Box::new(Ai::new()));
    g.play();
}
