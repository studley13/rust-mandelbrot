/*
 * tile.rs
 *
 * Renders the mandelbrot values for a tile at a given location and zoom.
 */

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

        // Simple antialiasing by adding extra row and column
        for y in 1..(resolution + 1) {
            for x in 1..(resolution + 1) {
                let point = self.get_point(x, y, resolution as f64);
                let depth = self.iterate(point, depth);
                //println!("({:+.5}, {:+.5}) = {:>6}", point.x, point.y, depth);
                render.result.push(depth);
            }
        }

        //println!("Data is {:?}", render.result);

        self.render = render;
    }

    pub fn point_value (&self, x: u32, y: u32) -> f64 {
        // Get four points, for aliasing
        let sum = self.actual_point_value(x    , y    )
                + self.actual_point_value(x + 1, y    )
                + self.actual_point_value(x    , y + 1)
                + self.actual_point_value(x + 1, y + 1);
        sum as f64 / (self.render.depth as f64 * 4f64)
    }

    fn get_point(&self, x: u32, y: u32, resolution: f64) -> Point {
        let size = 1f64 / ((1 << self.location.z) as f64);
        Point {
            x: self.location.x - ((resolution * size) / 2f64) + x as f64 * size,
            y: self.location.y - ((resolution * size) / 2f64) + y as f64 * size,
        }
    }

    fn iterate(&self, point: Point, max_depth: u32) -> u32 {
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

        /*
        if depth == max_depth {
            depth = 0;
        }
        */

        return depth;
    }

    fn actual_point_value(&self, x: u32, y: u32) -> u32 {
        let value_index = y * self.render.resolution + x;
        self.render.result[value_index as usize]
    }
}
