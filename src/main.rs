mod tile;

use std::env;
use std::process::exit;
use std::str::FromStr;

const MAX_DEPTH:          u32 = 1024;
const DEFAULT_RESOLUTION: u32 = 80;

fn main() {
    let loc = tile_requirements();

    println!("Tile is {:?}", loc);
    let mut tile = tile::Tile::new(loc);
    tile.render(MAX_DEPTH, DEFAULT_RESOLUTION);

    for y in 0..(DEFAULT_RESOLUTION - 1) {
        for x in 0..(DEFAULT_RESOLUTION- 1) {
            let color = (tile.point_value(x, y) * 15f64) as u32 + 0xF0;
            print!("\x1B[48;5;{}m \x1B[0m", color);
        }
        println!("");
    }
}

fn tile_requirements() -> tile::Location {
    let mut args = env::args();
    let mut loc = tile::Location {x: 0f64, y: 0f64, z: 0i64};
    
    if args.len() != 4 {
        usage();
    }

    // Consume program name
    args.next();

    // Get location
    loc.x = f64::from_str(args.next().unwrap().as_str()).unwrap();
    loc.y = f64::from_str(args.next().unwrap().as_str()).unwrap();
    loc.z = i64::from_str(args.next().unwrap().as_str()).unwrap();

    return loc;
}

fn usage() {
    println!("{prog} usage:", prog=env::args().min().unwrap());
    print!(r#"
{prog} center_x center_y zoom_factor

Arguments:
    center_x     the center of the tile's x position
    center_y     the center of the tile's y position
    zoom_factor  the zoom factor of the tile
"#, prog=env::args().min().unwrap());

    exit(1);
}
