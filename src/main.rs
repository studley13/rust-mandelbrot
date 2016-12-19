mod tile;
mod image;

use std::env;
use std::process::exit;
use std::str::FromStr;

const MAX_DEPTH:          u32 = 255;
const DEFAULT_RESOLUTION: u32 = 1024;

fn main() {
    let loc = tile_requirements();

    let mut tile = tile::Tile::new(loc);
    tile.render(MAX_DEPTH, DEFAULT_RESOLUTION);

    let mut img = image::Bitmap::new(DEFAULT_RESOLUTION as i32, DEFAULT_RESOLUTION as i32, 24).unwrap();

    for y in 0..DEFAULT_RESOLUTION {
        for x in 0..DEFAULT_RESOLUTION {
            let color = image::Color::Grey((tile.point_value(x, y) * MAX_DEPTH as f64) as u8);
            img.set(x as i32, y as i32, color);
        }
    }

    let file_name = format!("mbrotTile_{:.6}x-{:.6}y-{}z.bmp", loc.x, loc.y, loc.z);
    img.to_file(&file_name);
    println!("{}", file_name);
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
