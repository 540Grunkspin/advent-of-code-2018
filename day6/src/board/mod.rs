mod point;

use self::point::Point;
use std::collections::HashSet;

pub struct Board {
    board: Vec<Vec<i32>>,
    points: Vec<(i32, Point)>,
}

impl Board {
    pub fn new(points: Vec<(i32, Point)>) -> Board {
        let x_vals = points
            .iter()
            .map(|(_, Point { x, .. })| x)
            .collect::<Vec<&i32>>();

        let y_vals = points
            .iter()
            .map(|(_, Point { y, .. })| y)
            .collect::<Vec<&i32>>();

        let max_x = **x_vals.iter().max().unwrap() as usize;
        let max_y = **y_vals.iter().max().unwrap() as usize;

        let board = vec![vec![0; max_x + 2]; max_y + 2];

        return Board {
            points: points,
            board: board,
        };
    }

    fn find_closes_id(&self, point: &Point) -> i32 {
        let distances = self
            .points
            .iter()
            .map(move |(id, p)| (id, p.distance(point)))
            .collect::<Vec<(&i32, i32)>>();

        let min_distance = distances
            .iter()
            .map(|(_, distance)| distance)
            .min()
            .unwrap();

        let mins = distances
            .iter()
            .filter(|(_, distance)| distance == min_distance)
            .collect::<Vec<&(&i32, i32)>>();

        if mins.len() == 1 {
            let (id, _) = mins.iter().nth(0).unwrap();
            return **id;
        } else {
            return 0;
        }
    }

    pub fn mark_with_closest(&mut self) {
        let mut board = self.board.clone();
        for (y, lane) in board.iter_mut().enumerate() {
            *lane = lane
                .iter()
                .enumerate()
                .map(|(x, _)| self.find_closes_id(&Point::new(x as i32, y as i32)))
                .collect::<Vec<i32>>();
        }

        self.board = board;
    }

    pub fn get_point_areas(&self) -> Vec<(i32, i32)> {
        let mut infinate: HashSet<i32> = HashSet::new();
        for (y, lane) in self.board.iter().enumerate() {
            if y == 0 || y == self.board.len() - 1 {
                for &id in lane.iter() {
                    infinate.insert(id);
                }
            }
            infinate.insert(lane[0]);
            infinate.insert(lane[lane.len() - 1]);
        }

        let non_infinate_areas: Vec<(i32, i32)> = self
            .points
            .iter()
            .filter(|&(id, _)| !infinate.contains(id))
            .map(|&(point_id, _)| {
                (
                    point_id,
                    self.board
                        .iter()
                        .flatten()
                        .filter(|&&id| id == point_id)
                        .collect::<Vec<&i32>>()
                        .len() as i32,
                )
            })
            .collect();

        return non_infinate_areas;
    }

    pub fn area_closest_to_all(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();
        for (y, lane) in self.board.iter().enumerate() {
            for (x, _) in lane.iter().enumerate() {
                let point = Point::new(x as i32, y as i32);
                if self.distance_to_all(&point) < 10000 {
                    result.push(point);
                }
            }
        }

        return result;
    }

    fn distance_to_all(&self, point: &Point) -> i32 {
        self.points.iter().map(|(_, p)| p.distance(point)).sum()
    }
}

impl From<Vec<String>> for Board {
    fn from(input: Vec<String>) -> Board {
        let points = input
            .iter()
            .enumerate()
            .map(|(i, coord)| ((i + 1) as i32, Point::from(coord)))
            .collect::<Vec<(i32, Point)>>();

        return Board::new(points);
    }
}
