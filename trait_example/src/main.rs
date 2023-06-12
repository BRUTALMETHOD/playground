pub trait Total {
    fn total(&self) -> u32;
}

impl Total for Ball {
    fn total(&self) -> u32 {
        self.ball_volume
    }
}

impl Total for Egg {
    fn total(&self) -> u32 {
        self.egg_white + self.egg_yellow
    }
}
#[derive(Debug)]
struct Ball {
    ball_volume: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Egg {
    egg_white: u32,
    egg_yellow: u32,
}

fn main() {
    println!("Hello, world!");
    let ball = Ball { ball_volume: 10 };
    let egg = Egg {
        egg_white: 5,
        egg_yellow: 5,
    };
    println!("{:?}", ball);
    println!("{:?}", ball.total());
    println!("{:?}", egg);
    println!("{:?}", egg.total());
}
