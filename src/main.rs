use std::iter::Enumerate;
use Fraction::Fractions;

type F = fraction::Fraction;

fn main() {
    println!("Hello, world!");
    let cardinal:PathType = 10;                                                                        
    let diagonal:PathType = 14;
    let north: Path = Path{
        direction: Direction::North,
        alias: "North",
        path_type: cardinal,
        node: node{}
    };

    let mut paths: Vec<Path> = vec![{
        north = Path::north,
    },{
        direction = Direction::South,
        alias = "South",
        path_type = cardinal
    },{
        direction = Direction::East,
        alias = "East",
        path_type = cardinal
    },{
        direction = Direction::West,
        alias = "West",
        path_type = cardinal
    },{
        direction = Direction::NorthEast,
        alias = "NorthEast",
        path_type = diagonal
    },{
        direction = Direction::NorthWest,
        alias = "NorthWest",
        path_type = diagonal
    },{
        direction = Direction::SouthEast,
        alias = "SouthEast",
        path_type = diagonal
    },{
        direction = Direction::SouthWest,
        alias = "SouthWest",
        path_type = diagonal
    }]; 
}

struct Node {
    paths: Vec<Path>,
    indices: Indices,
    cover: Enumerate<u8>
}

struct Indices {
    x: u8,
    y: u8,
    z: u8,
}

struct Path {
    node: Node,
    direction: Direction,
    alias: String,
    path_type: PathType,
}

enum PathType {
    Cardinal(u8),
    Diagonal(u8),
}

enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

enum Cover {
    Full(F),
    TQ(F),
    Half(F),
    Zero(F),
}
