struct Position{
    x: u32,
    y: u32,
}

struct Mario{
    position: Position,
    coins: u32,
    power_up: Option<PowerUp>
}

impl Mario {
    fn jump(&mut self){
        self.position.y += 1;

    }
    
}

enum PowerUp{
    SuperMushroom,
    FireFlower,
    Starman,
    CapeFeather,
}

fn main() {
    // at the beginning of the game
    let position = Position{
        x: 0,
        y: 0,
    };

    let mario = Mario{
        position: position,
        coins: 0,
        power_up: None,
    };

}
