use gm::flat::point::PointBase;
use rand::seq::SliceRandom;
use tokio::sync::mpsc::{self, Receiver};

use crate::{Cell, Grid};

type Point = PointBase<i32>;

#[derive(Debug)]
pub struct Maker {
    size:        Point,
    current_pos: Point,
    stack:       Vec<Point>,
    grid:        Grid,
}

impl Maker {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size:        Point {
                x: width as _,
                y: height as _,
            },
            current_pos: Default::default(),
            stack:       Default::default(),
            grid:        vec![vec![Cell::default(); width]; height],
        }
    }

    pub fn generate(&mut self) -> Receiver<Grid> {
        #![allow(unused_must_use)]
        let (sender, receiver) = mpsc::channel::<Grid>(1);

        self.current_mut().visited = true;

        while self.has_unvisited() {
            let unvisited = self.unvisited_neighbours();

            if unvisited.is_empty() {
                if let Some(pop) = self.stack.pop() {
                    self.current_pos = pop;
                }
                continue;
            }

            dbg!(&unvisited);

            let next = *unvisited.choose(&mut rand::thread_rng()).unwrap();

            self.stack.push(self.current_pos);

            sender.send(self.grid.clone());

            self.remove_walls(next);

            self.current_pos = next;
            self.at_mut(next).visited = true;
        }

        receiver
    }

    fn make(mut self) -> Grid {
        self.generate();
        dbg!("goneroted");
        self.grid
    }
}

impl Maker {
    fn current_mut(&mut self) -> &mut Cell {
        self.at_mut(self.current_pos)
    }

    fn at(&self, pos: Point) -> &Cell {
        &self.grid[pos.x as usize][pos.y as usize]
    }

    fn at_mut(&mut self, pos: Point) -> &mut Cell {
        &mut self.grid[pos.x as usize][pos.y as usize]
    }

    fn has_unvisited(&self) -> bool {
        for x in &self.grid {
            for cell in x {
                if !cell.visited {
                    return true;
                }
            }
        }
        false
    }

    fn unvisited_neighbours(&self) -> Vec<Point> {
        let mut result = vec![];

        const NEIGH: &[Point; 4] = &[
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
        ];

        for neigh in NEIGH {
            let pos = &self.current_pos + neigh;
            if pos.is_negative() || self.at(pos).visited {
                continue;
            }
            if pos.x >= self.size.x - 1 || pos.y >= self.size.y - 1 {
                continue;
            }
            result.push(pos);
        }

        result
    }

    fn remove_walls(&mut self, pos: Point) {
        let current = self.current_pos;

        if current.x < pos.x {
            self.at_mut(current).rigth = false;
            self.at_mut(pos).left = false;
        } else if current.x > pos.x {
            self.at_mut(current).left = false;
            self.at_mut(pos).rigth = false;
        } else if current.y < pos.y {
            self.at_mut(current).down = false;
            self.at_mut(pos).up = false;
        } else if current.y > pos.y {
            self.at_mut(current).up = false;
            self.at_mut(pos).down = false;
        } else {
            panic!("BUG");
        }
    }
}

pub fn make(width: usize, height: usize) -> Grid {
    Maker::new(width, height).make()
}
