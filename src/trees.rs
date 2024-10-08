use nannou::{prelude::*};
use std::hash::{self, Hash};
use std::{cell::RefCell, rc::Rc};
// use std::cell::bo
use std::thread;

// TODO: MAKE UPDATE TREE!
pub struct Tree {
    pub nodes: Vec<Option<Node>>,
    pub points: Vec<Rc<RefCell<Vec2>>>,
    pub len: usize,
    // pub leaves: Vec<Rc<Node>>,
}

pub struct Node {
    min: Vec2,
    max: Vec2,
    points: Vec<Rc<RefCell<Vec2>>>, // vector of indices of points
    children: Option<usize>, // child sorteds as index, index+1, index+2, index+3
    existing_children: [bool; 4],
}

impl Tree {
    const MASK:u8 = 15; // 00001111
    pub fn new(min: Vec2, max: Vec2, points: Vec<Rc<RefCell<Vec2>>>) -> Self {
        Tree {
            nodes: Vec::with_capacity(1000),
            points,
            len: 0,
            // leaves: Vec::new(),
        }
    }
}


impl Node {
    pub fn new(min: Vec2, max: Vec2, points: Vec<Rc<RefCell<Vec2>>>) -> Self {
        Node {
            min,
            max,
            points,
            children:None,
            existing_children: [false, false, false, false,]
            // root,
        }
    }
}

impl NodeTrait for Node {
    fn point_inside(&self, point: &Vec2) -> bool {

        point.x < self.max.x && point.x > self.min.x && point.y < self.max.y && point.y > self.min.y
    }
    fn quarter(&self, point: &Rc<RefCell<Vec2>>) -> Vec2 {
        let middle_point = (self.min + self.max) / 2.0;

        let direction = *point.borrow() - middle_point;

        // which quarter is the point in the node?
        let mut quarter = Vec2::new(0.0, 0.0);

        if direction.x > 0.0 {
            quarter.x = 1.0;
        } else {
            quarter.x = -1.0;
        }

        if direction.y > 0.0 {
            quarter.y = 1.0;
        } else {
            quarter.y = -1.0;
        }

        quarter
    }

    fn add_point(&mut self, point: &Rc<RefCell<Vec2>>) {
        
    }
    /// converts the quarter to an index
    fn quarter_index(&self, quarter: Vec2) -> usize {
        ((quarter.x > 0.0) as usize) | (((quarter.y > 0.0) as usize) << 1)

    }
    /// converts the index to a quarter
    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error> {
        // index = 0 -> top left
        // index = 1 -> top right
        // index = 2 -> bottom left
        // index = 3 -> bottom right
        let quarter;

        match index {
            0 => quarter = Vec2::new(-1.0, 1.0),
            1 => quarter = Vec2::new(1.0, 1.0),
            2 => quarter = Vec2::new(-1.0, -1.0),
            3 => quarter = Vec2::new(1.0, -1.0),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "index is not valid",
                ));
            }
        }
        return Ok(quarter);
    }

    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error> {
        // quarter = Vec2::new(-1.0, 1.0) -> top left
        // quarter = Vec2::new(1.0, 1.0) -> top right
        // quarter = Vec2::new(-1.0, -1.0) -> bottom left
        // quarter = Vec2::new(1.0, -1.0) -> bottom right
        let quarter_node;
        let middle_point = (self.min + self.max) / 2.0;

        let create_node = |min: Vec2, max: Vec2| -> Result<Node, std::io::Error> {
            let new_points = Self::get_points_inside(&self.points, &min, &max);
            Ok(Node::new(min, max, new_points))
        };
        quarter_node = match quarter {
            q if q == vec2(-1.0, 1.0) => {
                let min = vec2(self.min.x, middle_point.y);
                let max = vec2(middle_point.x, self.max.y);
                create_node(min, max)?
            }
            q if q == vec2(1.0, 1.0) => {
                let min = middle_point;
                let max = self.max;
                create_node(min, max)?

            }
            q if q == vec2(-1.0, -1.0) => {
                let min = self.min;
                let max = middle_point;
                create_node(min, max)?

            }
            q if q == vec2(1.0, -1.0) => {
                let min = vec2(middle_point.x, self.min.y);
                let max = vec2(self.max.x, middle_point.y);
                create_node(min, max)?

            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No quarter is not valid",
                ));
            }
        };

        return Ok(quarter_node);
    }
    // draw using annou
    fn draw(&self, draw: &Draw) {
        let mid_points = (self.min + self.max) / 2.0;
        draw.rect()
            .xy(mid_points)
            .w_h(self.max.x - self.min.x, self.max.y - self.min.y)
            .stroke_weight(1.0)
            .color(rgb(1.0, self.points.len() as f32 / 20.0, 0.2));
    }
}

impl Node {
    fn get_points_inside(
        points: &[Rc<RefCell<Vec2>>],
        min: &Vec2,
        max: &Vec2,
    ) -> Vec<Rc<RefCell<Vec2>>> {
        points.iter()
            .filter(|point| {
                let p = point.borrow();
                p.x < max.x && p.x > min.x && p.y < max.y && p.y > min.y
            })
            .map(Rc::clone)
            .collect()
    }

    fn how_many_points_in_quarter(&self, quarter: Vec2) -> usize {
        let mut points = 0;
        for point in self.points.iter() {
            if !self.point_inside(&point.borrow()) {
                continue;
            }
            if self.is_inside_quarter(point, quarter) {
                points += 1;
            }
        }
        points
    }

    fn is_inside_quarter(&self, point: &Rc<RefCell<Vec2>>, quarter: Vec2) -> bool {
        
        let max_x;
        let min_x;
        let max_y;
        let min_y;

        match quarter {
            q if q == vec2(-1.0, 1.0) => {
                max_x = (self.min.x + self.max.x) / 2.0;
                min_x = self.min.x;
                max_y = self.max.y;
                min_y = (self.min.y + self.max.y) / 2.0;
            }
            q if q == vec2(1.0, 1.0) => {
                max_x = self.max.x;
                min_x = (self.min.x + self.max.x) / 2.0;
                max_y = self.max.y;
                min_y = (self.min.y + self.max.y) / 2.0;
            }
            q if q == vec2(-1.0, -1.0) => {
                max_x = (self.min.x + self.max.x) / 2.0;
                min_x = self.min.x;
                max_y = (self.min.y + self.max.y) / 2.0;
                min_y = self.min.y;
            }
            _ => {
                max_x = self.max.x;
                min_x = (self.min.x + self.max.x) / 2.0;
                max_y = (self.min.y + self.max.y) / 2.0;
                min_y = self.min.y;
            }
        }

        point.borrow().x < max_x
            && point.borrow().x > min_x
            && point.borrow().y < max_y
            && point.borrow().y > min_y
    }
}
impl NodeTrait for Tree {
    fn draw(&self, draw: &Draw) {
        self.nodes[0].as_ref().unwrap().draw(draw);
    }

    fn point_inside(&self, point: &Vec2) -> bool {
        self.nodes[0].as_ref().unwrap().point_inside(point)
    }

    fn quarter(&self, point: &Rc<RefCell<Vec2>>) -> Vec2 {
        self.nodes[0].as_ref().unwrap().quarter(point)
    }

    fn quarter_index(&self, quarter: Vec2) -> usize {
        self.nodes[0].as_ref().unwrap().quarter_index(quarter)
    }

    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error> {
        Node::index_quarter(index)
    }

    fn add_point(&mut self, point: &Rc<RefCell<Vec2>>)
    /*, condition: C)
    where C: Fn(&Vec<Rc<RefCell<Vec2>>>) -> bool*/
    {
        if !self.point_inside(&point.borrow()) {
            return;
        }

        // self.points.push(point.clone());
        let mut current_node = self.nodes.first_mut().unwrap().as_mut().unwrap();
        // TODO: OPTIMIZE!!! AND FIX
        while current_node.point_inside(&point.borrow()) {
            let quarter: Vec2 = current_node.quarter(&point);
            if current_node.how_many_points_in_quarter(quarter) < 5 {
                break;
            }

            let index = current_node.quarter_index(quarter);

            if current_node.children.is_none() && current_node.existing_children[index] {
                current_node.children = Some(self.len - 1);
                current_node.existing_children[index] = true;
                for i in 0..4 {
                    if i == index {
                        self.nodes.push(Some(current_node.add_node(quarter).expect("no quarter found")));
                        continue;
                    }
                    self.nodes.push(None);
                }

            } else {
                self.nodes[current_node.children.unwrap() + index] = Some(current_node.add_node(quarter).expect("no quarter found"));
            }
            current_node = self.nodes[current_node.children.unwrap() + index].as_mut().unwrap();
            current_node.points.push(point.clone());
        }
    }

    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error> {
        self.nodes[0].as_ref().unwrap().add_node(quarter)
    }
}

pub trait NodeTrait {
    fn point_inside(&self, point: &Vec2) -> bool;
    fn quarter(&self, point: &Rc<RefCell<Vec2>>) -> Vec2;
    fn quarter_index(&self, quarter: Vec2) -> usize;
    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error>;
    fn add_point(&mut self, point: &Rc<RefCell<Vec2>>);
    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error>;
    fn draw(&self, draw: &Draw);
}

impl Tree {
    pub fn size(&self) -> usize {
        self.nodes[0].as_ref().unwrap().points.len()
    }

    pub fn draw_points(&self, draw: &Draw) {
        for point in self.nodes[0].as_ref().unwrap().points.iter() {
            let point = point.borrow();
            draw.ellipse().xy(*point).w_h(5.0, 5.0).color(BLACK);
        }
    }

    pub fn update(&mut self) {
        // self.nodes.truncate(1);
        // let self_borrow = self.nodes[0].as_mut().unwrap();


        // self_borrow.points.clear();

        // for point in &mut self.points.iter() {
        //     self.add_point(&point);
        // }
        
    }
}
