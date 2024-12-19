use fake::Fake;
use gm::flat::Size;

use crate::maze::{Cell, Grid};

type Point = gm::flat::Point<i32>;

#[derive(Debug)]
pub struct Maker {
    size:        Size<usize>,
    current_pos: Point,
    stack:       Vec<Point>,
    grid:        Grid,
}

impl Maker {
    fn new(size: Size<usize>) -> Self {
        assert!(!size.has_no_area());
        Self {
            size,
            current_pos: Point::default(),
            stack: vec![],
            grid: vec![vec![Cell::default(); size.height]; size.width],
        }
    }

    pub fn generate(size: Size<usize>) -> Grid {
        let mut maker = Maker::new(size);
        maker.add_missing_side();

        maker.current_mut().visited = true;

        while maker.has_unvisited() {
            let unvisited = maker.unvisited_neighbours();

            if unvisited.is_empty() {
                if let Some(pop) = maker.stack.pop() {
                    maker.current_pos = pop;
                }
                continue;
            }

            let next = unvisited[(0..unvisited.len()).fake::<usize>()];

            maker.stack.push(maker.current_pos);

            maker.remove_walls(next);

            maker.current_pos = next;
            maker.at_mut(next).visited = true;
        }

        maker.grid
    }
}

impl Maker {
    fn add_missing_side(&mut self) {
        for i in 0..self.size.height {
            self.grid[0][i].left = true;
        }

        for i in 0..self.size.width {
            self.grid[i][0].bottom = true;
        }
    }

    fn current_mut(&mut self) -> &mut Cell {
        self.at_mut(self.current_pos)
    }

    fn at(&self, pos: Point) -> &Cell {
        &self.grid[usize::try_from(pos.x).unwrap()][usize::try_from(pos.y).unwrap()]
    }

    fn at_mut(&mut self, pos: Point) -> &mut Cell {
        &mut self.grid[usize::try_from(pos.x).unwrap()][usize::try_from(pos.y).unwrap()]
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
        const NEIGHBOURS: &[Point; 4] = &[
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
        ];

        let mut result = vec![];

        for neighbor in NEIGHBOURS {
            let pos = &self.current_pos + neighbor;
            if pos.x >= self.size.width.try_into().unwrap() || pos.y >= self.size.height.try_into().unwrap() {
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
            self.at_mut(current).right = false;
            self.at_mut(pos).left = false;
        } else if current.x > pos.x {
            self.at_mut(current).left = false;
            self.at_mut(pos).right = false;
        } else if current.y > pos.y {
            self.at_mut(current).bottom = false;
            self.at_mut(pos).top = false;
        } else if current.y < pos.y {
            self.at_mut(current).top = false;
            self.at_mut(pos).bottom = false;
        } else {
            panic!("BUG");
        }
    }
}
