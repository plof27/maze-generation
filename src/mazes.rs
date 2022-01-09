use image::{Rgb, RgbImage};
use log::{info, debug};
use rand::{thread_rng, prelude::SliceRandom};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    Path
}

type CellIndex = (usize, usize);

pub struct Maze {
    cells: Vec<Vec<CellType>>,
    size: (usize, usize),
}

impl Maze {
    /// Generates a new maze using Wilson's Algorithm
    /// Sizes must be odd
    pub fn new(x_size: usize, y_size: usize) -> Result<Self, &'static str> {
        info!("Starting maze generation");

        if x_size % 2 == 0 || y_size % 2 == 0 {
            return Err("Maze sizes must be odd numbers")
        }

        let mut maze = Maze {
            cells: vec![vec![CellType::Wall; y_size]; x_size],
            size: (x_size, y_size)
        };

        // This is arbitrary. All that matters is that we pick one cell that is (odd, odd) to be the "seed"
        maze.cells[1][1] = CellType::Path;
        info!("Initial cell: (1, 1)");
        
        // Cells that must be included in the maze eventually
        // These are used both for checking if the maze is done, and for picking starting points for the random walk
        let mut necessary_cells: Vec<CellIndex> = Vec::new();
        for i in (1..maze.size.0).step_by(2) {
            for j in (1..maze.size.1).step_by(2) {
                necessary_cells.push((i, j));
            }
        }

        // This shuffle is totally unnecessary, and probably makes the algorithm slower.
        // But it *looks cool*.
        necessary_cells.shuffle(&mut thread_rng());

        for walk_start_point in necessary_cells {
            if maze.cells[walk_start_point.0][walk_start_point.1] == CellType::Wall {
                let walk = maze.generate_loop_erased_random_walk(walk_start_point);
                for cell in walk {
                    maze.cells[cell.0][cell.1] = CellType::Path;
                }
            }
        }

        info!("Maze generation complete");

        Ok(maze)
    }

    /// Generates a loop erased random walk through the maze walls, obeying rules about path separation
    /// Used when generating a new maze
    fn generate_loop_erased_random_walk(&mut self, starting_point: CellIndex) -> Vec<CellIndex> {
        info!("Starting random walk at: ({}, {})", starting_point.0, starting_point.1);

        let mut random_walk = Vec::<CellIndex>::new();
        let mut current_pos = starting_point;
        
        let mut rng = thread_rng();
        
        random_walk.push(current_pos);

        // Take random steps until we reach a piece of existing maze
        while self.cells[current_pos.0][current_pos.1] == CellType::Wall {
            let candidate_points = self.generate_candidate_cells(current_pos);
            
            let step = *candidate_points.choose(&mut rng).unwrap(); // Safe to unwrap since we know candidate_points will always have at least 2 options

            random_walk.push(step[0]);
            random_walk.push(step[1]);
            current_pos = step[1];
        }

        debug!("Random walk generated: {:?}", random_walk);

        // Erase loops
        // Need to do weird loop stuff here because we're snipping things out of random_walk while iterating through it
        debug!("Starting loop erasure");
        let mut i: usize = 0;
        while i < random_walk.len() {
            debug!("Adding to loop erased walk: {:?}", random_walk[i]);
            for j in ((i+1)..(random_walk.len()-1)).rev() {
                debug!("Checking index: {}", j);
                if random_walk[i].0 == random_walk[j].0 && random_walk[i].1 == random_walk[j].1 {
                    debug!("Erasing range: {}..{}", i, j);
                    random_walk.drain(i..j);
                    break;
                }
            }
            i += 1;
        }

        debug!("Random loop-erased walk generated: {:?}", random_walk);

        random_walk
    }

    /// Generates valid cells to step to during random walks, given a cell to be stepping from
    /// We always step two cells in a direction at a time, so that our walls stay nicely separated
    /// TODO: This can be made more efficient by nesting checks cleverly
    fn generate_candidate_cells(&self, current_pos: CellIndex) -> Vec<[CellIndex; 2]> {
        let mut candidate_points: Vec<[CellIndex; 2]> = Vec::with_capacity(4);
        if current_pos.0 > 1 {
            candidate_points.push([(current_pos.0 - 1, current_pos.1), (current_pos.0 - 2, current_pos.1)]); // Left
        }
        if current_pos.0 < self.size.0 - 2 {
            candidate_points.push([(current_pos.0 + 1, current_pos.1), (current_pos.0 + 2, current_pos.1)]); // Right
        }
        if current_pos.1 > 1 {
            candidate_points.push([(current_pos.0, current_pos.1 - 1), (current_pos.0, current_pos.1 - 2)]); // Up
        }
        if current_pos.1 < self.size.1 - 2 {
            candidate_points.push([(current_pos.0, current_pos.1 + 1), (current_pos.0, current_pos.1 + 2)]); // Down
        }

        candidate_points
    }

    /// Builds an image for the maze. Every cell in the maze is 1 pixel. Walls are black, paths are white.
    pub fn build_image(&self) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
        info!("Starting image generation");

        let mut img = RgbImage::new(self.size.0 as u32, self.size.1 as u32);

        for (i, col) in self.cells.iter().enumerate() {
            for (j, cell) in col.iter().enumerate() {
                let color = if cell == &CellType::Wall {
                    Rgb([0, 0, 0])
                } else {
                    Rgb([255, 255, 255])
                };

                img.put_pixel(i as u32, j as u32, color);
            }
        }

        info!("Image generation complete");

        img
    }
}
