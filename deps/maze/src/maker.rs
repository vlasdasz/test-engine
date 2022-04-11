use std::time::Duration;

use gm::flat::{PointBase, SizeBase};
use rand::seq::SliceRandom;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver},
    time::sleep,
};

use crate::{Cell, Grid};

type Size = SizeBase<i32>;
type Point = PointBase<i32>;

#[derive(Debug)]
pub struct Maker {
    size:        Size,
    current_pos: Point,
    stack:       Vec<Point>,
    grid:        Grid,
}

impl Maker {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size:        Size {
                width:  width as _,
                height: height as _,
            },
            current_pos: Default::default(),
            stack:       Default::default(),
            grid:        vec![vec![Cell::default(); height]; width],
        }
    }

    pub fn generate(width: usize, height: usize) -> UnboundedReceiver<Grid> {
        let (sender, receiver) = mpsc::unbounded_channel::<Grid>();

        tokio::spawn(async move {
            let mut maker = Maker::new(width, height);

            maker.current_mut().visited = true;

            while maker.has_unvisited() {
                let unvisited = maker.unvisited_neighbours();

                if unvisited.is_empty() {
                    if let Some(pop) = maker.stack.pop() {
                        maker.current_pos = pop;
                    }
                    continue;
                }

                let next = *unvisited.choose(&mut rand::thread_rng()).unwrap();

                maker.stack.push(maker.current_pos);

                sender.send(maker.grid.clone()).unwrap();

                sleep(Duration::from_millis(5)).await;

                maker.remove_walls(next);

                maker.current_pos = next;
                maker.at_mut(next).visited = true;
            }
            sender.send(maker.grid.clone()).unwrap();
        });

        receiver
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

        const NEIGHBOURS: &[Point; 4] = &[
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
        ];

        for neighbor in NEIGHBOURS {
            let pos = &self.current_pos + neighbor;
            if pos.x >= self.size.width || pos.y >= self.size.height {
                continue;
            }
            if pos.is_negative() || self.at(pos).visited {
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
