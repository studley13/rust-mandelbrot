/*
 * tile.rs
 *
 * Renders the mandelbrot values for a tile at a given location and zoom.
 */

extern crate threadpool;

use self::threadpool::ThreadPool;
use std::sync::mpsc::channel;

const NUM_THREADS: usize = 4;

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: i64,
}

#[derive(Debug)]
pub struct Tile {
    location: Location,
    render:   Render,
}

#[derive(Debug)]
struct Render {
    resolution: u32,
    depth:      u32,
    result:     Vec<u32>,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64
}

impl Tile {
    pub fn new (loc: Location) -> Tile {
        Tile {
            location: loc,
            render:   Render{
                resolution: 0u32,
                depth:      0u32,
                result:     Vec::new(),
            }
        }
    }

    pub fn render (&mut self, depth: u32, resolution: u32) {
        let mut render = Render {
            resolution: resolution,
            depth:      depth,
            result:     Vec::new(),
        };

        let pool = ThreadPool::new(NUM_THREADS);
        let (tx, rx) = channel();

        let jobs: Vec<(Point, &u32, u32)> = Vec::new();

        // Simple antialiasing by adding extra row and column
        for y in 0..(resolution + 1) {
            for x in 0..(resolution + 1) {
                render.result.push(0); // Dummy value
                let pos = render.result.len() - 1;
                let point = self.get_point(x, y, resolution as f64);
                let inner_depth = depth;
                let tx = tx.clone();
                pool.execute(move || {
                    //tx.send((Tile::iterate(point, inner_depth), pos)).unwrap();
                    tx.send((Tile::iterate(point, inner_depth), pos));
                });
            }
        }

        //println!("Data is {:?}", render.result);
        for y in 1..(resolution + 1) {
            for x in 1..(resolution + 1) {
                let (val, pos) = rx.recv().unwrap();
                render.result[pos] = val;
                //println!("{} @ {}", val, pos);
            }
        }

        self.render = render;
    }

    pub fn point_value (&self, x: u32, y: u32) -> f64 {
        // Get four points, for aliasing
        let sum = self.actual_point_value(x    , y    )
                + self.actual_point_value(x + 1, y    )
                + self.actual_point_value(x    , y + 1)
                + self.actual_point_value(x + 1, y + 1);
        sum as f64 / (self.render.depth as f64 * 4f64)
        //self.actual_point_value(x, y) as f64 / self.render.depth as f64
    }

    fn get_point(&self, x: u32, y: u32, resolution: f64) -> Point {
        let size = 4f64 / ((1u64 << self.location.z) as f64 * resolution);
        Point {
            x: self.location.x - ((resolution * size) / 2f64) + x as f64 * size,
            y: self.location.y - ((resolution * size) / 2f64) + y as f64 * size,
        }
    }

    fn iterate(point: Point, max_depth: u32) -> u32 {
        let mut depth = 0u32;

        let mut next = Point {x: 0f64, y: 0f64};
        let mut curr = next;

        while curr.x * curr.x + curr.y * curr.y < 4f64 && depth < max_depth {
            next = Point {
                x: curr.x * curr.x - curr.y * curr.y + point.x,
                y: 2f64 * curr.x * curr.y + point.y
            };
            curr = next;
            depth += 1;
        }

        
        if depth == max_depth {
            depth = 0;
        }
       

        return depth;
    }

    fn actual_point_value(&self, x: u32, y: u32) -> u32 {
        // +1 to accomodate the aliasing feature
        let value_index = y * (self.render.resolution + 1) + x;
        self.render.result[value_index as usize]
    }
}
