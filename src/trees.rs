use nannou::{prelude::*};
use rand::Error;
use std::fmt::Debug;
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

// #[derive(Debug)]
pub struct Node {
    min: Vec2,
    max: Vec2,
    points: Vec<Rc<RefCell<Vec2>>>, // vector of indices of points
    children: Option<usize>, // child sorteds as index, index+1, index+2, index+3
    existing_children: [bool; 4],
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_str(format!("min: {}, max: {}", self.min, self.max/* , self.points*/).trim())
        f.write_str("NAME")
    }
}

impl Tree {
    const MASK:u8 = 15; // 00001111
    pub fn new(min: Vec2, max: Vec2, points: Vec<Rc<RefCell<Vec2>>>) -> Self {
        let node = Some(Node::new(min, max, points.iter().map(|p| p.clone()).collect()));
        let mut tree = Tree {
            nodes: Vec::with_capacity(1000),
            points,
            len: 1,
            // leaves: Vec::new(),
        };

        tree.nodes.push(node);

        tree
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
            let new_points: Vec<Rc<RefCell<Vec2>>> = Self::get_points_inside(&self.points, &min, &max);
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
        for node in &self.nodes{
            if let Some(node) = node {
                node.draw(draw);
            }
        } 
        
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
    {
        if !self.point_inside(&point.borrow()) {
            return;
        }

        let mut current_node = Some(self.nodes.first_mut().unwrap().as_mut().unwrap());

        current_node.as_mut().unwrap().points.push(point.clone());
        // TODO: OPTIMIZE!!! AND FIX BUG  
        while current_node.as_ref().unwrap().point_inside(&point.borrow()) {
            let quarter: Vec2 = current_node.as_ref().unwrap().quarter(&point);
            if current_node.as_ref().unwrap().how_many_points_in_quarter(quarter) < 10 {
                break;
            }
            let index = current_node.as_ref().unwrap().quarter_index(quarter);
            
            // if you dont have children add children
            if current_node.as_ref().unwrap().children.is_none()  {
                let current_node_unwrap = current_node.as_mut().unwrap();

                current_node_unwrap.children = Some(self.len);

                self.len+=4;


                current_node_unwrap.existing_children[index] = true;

                let current_child_index = current_node_unwrap.children.unwrap() + index;

                let node = Some(current_node_unwrap.add_node(quarter).expect("no quarter found"));

                current_node = None;

                
                self.nodes.append(&mut Self::choose_nodes(node, index).unwrap());


                current_node = self.nodes[current_child_index].as_mut();

            } else if !current_node.as_ref().unwrap().existing_children[index] { // if child is None, add the child.
                let current_node_unwrap = current_node.as_mut().unwrap();
                current_node_unwrap.existing_children[index] = true;
                let node = Some(current_node_unwrap.add_node(quarter).expect("no quarter found"));
                let current_child_index = current_node.as_ref().unwrap().children.unwrap() + index;

                current_node = None;

                self.nodes[current_child_index] = node;
                current_node = self.nodes[current_child_index].as_mut();
            } else {
                let current_child_index = current_node.as_ref().unwrap().children.unwrap() + index;
                self.nodes[current_child_index].as_mut().unwrap().points.push(point.clone());
                current_node = self.nodes[current_child_index].as_mut();
            }
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
        self.nodes.truncate(1);
        self.len = 1;
        // println!("{:?}", self.nodes);
        let self_borrow = self.nodes[0].as_mut().unwrap();


        self_borrow.points.clear();
        self_borrow.children = None;
        self_borrow.existing_children = [false, false, false, false];


        for point in 0..self.points.len() {
            self.add_point(&self.points[point].clone());
        }
        
    }

    pub fn choose_nodes(node: Option<Node>, index: usize)-> Result<Vec<Option<Node>>, String>{
        match index {
            0 => {
                Ok(vec![node, None, None, None])
            }
            1 => {
                Ok(vec![None, node, None, None])
                
            }
            2 => {
                Ok(vec![None, None, node, None])
                
            }
            3 => {
                Ok(vec![None, None, None, node])
                
            }
            _ => {
                Err(String::from("index out of range"))
            }
        }
    }
}
