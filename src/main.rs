struct Position{
    x: u32,
    y: u32,
}

struct Mario{
    position: Position,
    coins: u32,
    power_up: Option<PowerUp>
}

enum PowerUp{
    SuperMushroom,
    FireFlower,
    Starman,
    CapeFeather,
}

fn main() {
    println!("Hello, world!");
}
