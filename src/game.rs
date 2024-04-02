use gloo::console::log;
use rand::distributions::Distribution;

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub color: String,
    pub probability: f32,
    pub visited: bool,
}

impl Cell {
    pub fn new(x: i32, y: i32, color: String, probability: f32) -> Cell {
        Cell {
            x,
            y,
            color,
            probability,
            visited: false,
        }
    }
}

#[derive(Clone, Debug)]
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
            yellow,
            orange,
            red,
            // yellow: yellow + green,
            // orange: orange + yellow + green,
            // red: red + orange + yellow + green,
        }
    }
}

pub fn relative_direction(x: i32, y: i32, ghost_x: i32, ghost_y: i32) -> String {
    if ghost_x < x {
        if ghost_y < y {
            "NW".to_string()
        } else if ghost_y > y {
            "NE".to_string()
        } else {
            "N".to_string()
        }
    } else if ghost_x > x {
        if ghost_y < y {
            "SW".to_string()
        } else if ghost_y > y {
            "SE".to_string()
        } else {
            "S".to_string()
        }
    } else {
        if ghost_y < y {
            "W".to_string()
        } else if ghost_y > y {
            "E".to_string()
        } else {
            "BINGO!".to_string()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub grid: Vec<Vec<Cell>>,
    pub ghost_position: (i32, i32),
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
            ConditionalProbabilities::new(0, 0.05, 0.05, 0.10, 0.80),
            ConditionalProbabilities::new(1, 0.05, 0.10, 0.75, 0.10),
            ConditionalProbabilities::new(2, 0.05, 0.10, 0.75, 0.10),
            ConditionalProbabilities::new(3, 0.10, 0.70, 0.15, 0.05),
            ConditionalProbabilities::new(4, 0.10, 0.70, 0.15, 0.05),
            ConditionalProbabilities::new(5, 0.70, 0.10, 0.10, 0.05),
        ];
        Game {
            grid,
            ghost_position: (0, 0),
            score: 30,
            busts: 2,
            conditional_probabilities,
        }
    }

    pub fn reset(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                self.grid[x as usize][y as usize].color = "white".to_string();
                self.grid[x as usize][y as usize].probability = 0.0;
                self.grid[x as usize][y as usize].visited = false;
            }
        }
        self.score = 30;
        self.busts = 2;
        self.place_ghost();
        self.compute_initial_prior_probabilities();
    }

    pub fn place_ghost(&mut self) {
        let x = (rand::random::<i32>()).abs() % self.grid.len() as i32;
        let y = (rand::random::<i32>()).abs() % self.grid[0].len() as i32;
        self.ghost_position = (x, y);
    }

    pub fn compute_initial_prior_probabilities(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                self.grid[x as usize][y as usize].probability =
                    1.0 / (self.grid.len() * self.grid[0].len()) as f32;
            }
        }
    }

    pub fn distance_sense(&mut self, x: i32, y: i32) -> (String, String) {
        self.score -= 1;
        // Distance needs to be between 0 and 5
        let distance = (self.ghost_position.0 - x).abs() + (self.ghost_position.1 - y).abs();

        let distance = if distance > 5 { 5 } else { distance };

        // let between = rand::distributions::Uniform::from(0.0..1.0);
        // let mut rng = rand::thread_rng();
        // let random_number = between.sample_iter(&mut rng).next().unwrap();
        let choices = ["green", "yellow", "orange", "red"];
        let weights = [
            self.conditional_probabilities[distance as usize].green * 100.0,
            self.conditional_probabilities[distance as usize].yellow * 100.0,
            self.conditional_probabilities[distance as usize].orange * 100.0,
            self.conditional_probabilities[distance as usize].red * 100.0,
        ];
        log!(&format!("Weights: {:?}", weights));
        let dist = rand::distributions::WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();
        let random_color = choices[dist.sample(&mut rng)];
        log!("Random number: {}, distance: {}", random_color, distance);
        log!(&format!("Ghost position: {}, {}", self.ghost_position.0, self.ghost_position.1));

        // Get direction of ghost relative to the cell (NE, NW, SE, SW)
        
        let direction = relative_direction(x, y, self.ghost_position.0, self.ghost_position.1);

        (random_color.to_string(), direction.to_string())

    }

    pub fn bust_ghost(&mut self, x: i32, y: i32) -> i8 {
        self.busts -= 1;
        if self.ghost_position.0 == x && self.ghost_position.1 == y {
            return 1;
        } else if self.busts == 0 {
            return 0;
        } else {
            return -1;
        }
    }

    pub fn update_posterior_ghost_location_probabilities(&mut self, color: String, x: i32, y: i32, g_direction: String) {
        // Update the probabilities of the ghost being in each cell based on the color sensed in the cell (x, y)
        // and the other sensed colors in the grid

        let mut sum = 0.0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                // Get the (i, j) cell's distance from the ghost position based on its distance from the (x, y) cell
                // and the distance between the (x, y) cell and the ghost position (given by the
                // color)

                let dist = (x - i as i32).abs() + (y - j as i32).abs();

                let index = match self
                    .conditional_probabilities
                    .iter()
                    .position(|p| p.distance == dist)
                {
                    Some(i) => i,
                    None => 5,
                };
                let prior = self.grid[i as usize][j as usize].probability;
                let likelihood = if color == "green" {
                    self.conditional_probabilities[index].green
                } else if color == "yellow" {
                    self.conditional_probabilities[index].yellow
                } else if color == "orange" {
                    self.conditional_probabilities[index].orange
                } else {
                    self.conditional_probabilities[index].red
                };
                let direction = relative_direction(x, y, i as i32, j as i32);
                let directed_likelihood = match direction.as_str() == g_direction.as_str() {
                    true => 0.75,
                    false => 0.25,
                };
                self.grid[i as usize][j as usize].probability = prior * likelihood * directed_likelihood;
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
