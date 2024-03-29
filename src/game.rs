use rand::Rng;

#[derive(Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub color: String,
    pub probability: f32,
}

impl Cell {
    pub fn new(x: i32, y: i32, color: String, probability: f32) -> Cell {
        Cell {
            x,
            y,
            color,
            probability,
        }
    }
}

#[derive(Clone)]
pub struct ConditionalProbabilities {
    distance: i32,
    green: f32,
    yellow: f32,
    orange: f32,
    red: f32,
}

impl ConditionalProbabilities {
    pub fn new(
        distance: i32,
        green: f32,
        yellow: f32,
        orange: f32,
        red: f32,
    ) -> ConditionalProbabilities {
        ConditionalProbabilities {
            distance,
            green,
            yellow: yellow + green,
            orange: orange + yellow + green,
            red: red + orange + yellow + green,
        }
    }
}

#[derive(Clone)]
pub struct Game {
    pub grid: Vec<Vec<Cell>>,
    ghost_position: (i32, i32),
    pub score: i32,
    pub busts: i32,
    pub conditional_probabilities: Vec<ConditionalProbabilities>,
}

// implement the clone trait for the Game struct


impl Game {
    pub fn new(w: i32, h: i32) -> Game {
        let mut grid = vec![];
        for x in 0..w {
            let mut row = vec![];
            for y in 0..h {
                row.push(Cell::new(x, y, "white".to_string(), 0.0));
            }
            grid.push(row);
        }
        let conditional_probabilities = vec![
            ConditionalProbabilities::new(0, 0.05, 0.10, 0.20, 0.65),
            ConditionalProbabilities::new(1, 0.05, 0.15, 0.60, 0.20),
            ConditionalProbabilities::new(2, 0.05, 0.15, 0.60, 0.20),
            ConditionalProbabilities::new(3, 0.20, 0.60, 0.15, 0.05),
            ConditionalProbabilities::new(4, 0.20, 0.60, 0.15, 0.05),
            ConditionalProbabilities::new(5, 0.60, 0.20, 0.10, 0.05),
        ];
        Game {
            grid,
            ghost_position: (0, 0),
            score: 30,
            busts: 2,
            conditional_probabilities,
        }
    }

    pub fn place_ghost(&mut self) {
        let x = rand::random::<i32>() % self.grid.len() as i32;
        let y = rand::random::<i32>() % self.grid[0].len() as i32;
        self.ghost_position = (x, y);
    }

    pub fn compute_initial_prior_probabilities(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                self.grid[x as usize][y as usize].probability = 1.0 / (self.grid.len() * self.grid[0].len()) as f32;
            }
        }
    }

    pub fn distance_sense(&self, x: i32, y: i32) -> String {
        let distance = (x - self.ghost_position.0).abs() + (y - self.ghost_position.1).abs();
        // Return a random color based on the conditional probabilities
        // for the distance between the ghost and the cell (x, y)
        // The color is chosen based on the probabilities in the table above
        // For example, if the distance is 0, the color is chosen based on the probabilities
        // in the first row of the table above

        let random_number = rand::random::<f32>();
        // let cumulative_probability = 0.0;
        let index = match self
            .conditional_probabilities
            .iter()
            .position(|p| p.distance == distance)
        {
            Some(i) => i,
            None => 5,
        };
        if random_number < self.conditional_probabilities[index].green {
            "green".to_string()
        } else if random_number < self.conditional_probabilities[index].yellow {
            "yellow".to_string()
        } else if random_number < self.conditional_probabilities[index].orange {
            "orange".to_string()
        } else {
            "red".to_string()
        }
    }

    pub fn bust_ghost(&mut self, x: i32, y: i32) -> bool {
        self.busts -= 1;
        if self.ghost_position.0 == x && self.ghost_position.1 == y {
            true
        } else {
            false
        }
    }

    pub fn update_posterior_ghost_location_probabilities(&mut self, color: String, x: i32, y: i32) {
        // Update the probabilities of the ghost being in each cell based on the color sensed in the cell (x, y)
        // and the other sensed colors in the grid

        let mut sum = 0.0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                // let distance = (i as i32 - x as i32).abs() + ((j as i32) - y as i32).abs();
                let prior_probability = self.grid[i as usize][j as usize].probability;
                let mut likelihood = 0.0;
                if self.grid[i as usize][j as usize].color == color {
                    likelihood = 1.0;
                }
                self.grid[i as usize][j as usize].probability = prior_probability * likelihood;
                sum += self.grid[i as usize][j as usize].probability;
            }
        }
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                self.grid[i as usize][j as usize].probability /= sum;
            }
        }
    }
}
