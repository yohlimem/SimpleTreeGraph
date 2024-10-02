use geom::point;
use nannou::{glam::Vec2Swizzles, prelude::*};
use std::{borrow::Borrow, rc::Rc};

pub struct Tree {
    pub node: Option<Node>,
}

pub struct Node {
    min: Vec2,
    max: Vec2,
    points: Vec<Rc<Vec2>>,
    children: Box<[Option<Node>; 4]>,
    length: usize,
}


impl Tree {
    pub fn new(min: Vec2, max: Vec2) -> Self{
        Tree {
            node: Some(Node::new(min, max, Vec::new())),
        }
    }
}

impl Node {
    pub fn new(min: Vec2, max: Vec2, points: Vec<Rc<Vec2>>) -> Self{
        Node {
            min,
            max,
            points,
            children: Box::new([None, None, None, None]),
            length: 0,
        }
    }
}

impl NodeTrait for Node{
    fn point_inside(&self, point:&Rc<Vec2>) -> bool{
        point.x < self.max.x && point.x > self.min.x && point.y < self.min.y && point.y > self.min.y
    }
    fn quarter(&self, point:&Rc<Vec2>) -> Vec2{
        let middle_point = (self.min + self.max)/2.0;

        let direction = (point.xy() - middle_point).normalize();
 
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
    // TODO: add node
    fn add_point<C>(&mut self, point: Rc<Vec2>, condition: C)
    where C: Fn(&Vec<Rc<Vec2>>) -> bool{
        let quarter = self.quarter(&point);
        let index = self.quarter_index(quarter);

        println!("{:?}", self.points);
        self.points.push(point.clone());
        if !condition(&self.points){
            if self.children[index].is_none() {
                let mut new_node = self.add_node(quarter).expect("no quarter found");
                new_node.add_point(Rc::clone(&point), condition);
                self.children[index] = Some(new_node);
                return;
            }
            self.children[index].as_mut().unwrap().add_point(Rc::clone(&point), condition);
        }
        
        
    }
    /// converts the quarter to an index
    fn quarter_index(&self, quarter: Vec2) -> usize{
        // index = 0 -> top left
        // index = 1 -> top right
        // index = 2 -> bottom left
        // index = 3 -> bottom right
        let mut index = 0;

        if quarter.x > 0.0 {
            index += 1;
        }

        if quarter.y > 0.0 {
            index += 2;
        }

        index
    }
    /// converts the index to a quarter
    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error>{
        // index = 0 -> top left
        // index = 1 -> top right
        // index = 2 -> bottom left
        // index = 3 -> bottom right
        let mut quarter = Vec2::new(0.0, 0.0);
        if index == 0 {
            quarter = Vec2::new(-1.0, 1.0);
        } else if index == 1 {
            quarter = Vec2::new(1.0, 1.0);
        } else if index == 2 {
            quarter = Vec2::new(-1.0, -1.0);
        } else if index == 3 {
            quarter = Vec2::new(1.0, -1.0);
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "index is not valid"));
        }
        return Ok(quarter);
    }

    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error>{
        // quarter = Vec2::new(-1.0, 1.0) -> top left
        // quarter = Vec2::new(1.0, 1.0) -> top right
        // quarter = Vec2::new(-1.0, -1.0) -> bottom left
        // quarter = Vec2::new(1.0, -1.0) -> bottom right
        let quarter_node;
        let middle_point = (self.min + self.max)/2.0;
        let mut position = Vec2::new(0.0, 0.0);
        let mut points_inside: Vec<Rc<Vec2>> = Vec::new();
        if quarter == vec2(-1.0, 1.0) {
            let min = vec2(self.min.x, middle_point.y);
            let max = vec2(middle_point.x, self.max.y);
            let new_points = Self::get_points_inside(&self.points, min, max);
            quarter_node = Node::new(min, max, new_points);
        } else if quarter == vec2(1.0, 1.0) {
            let min = middle_point;
            let max = self.max;
            let new_points = Self::get_points_inside(&self.points, min, max);
            quarter_node = Node::new(min, max, new_points);
        } else if quarter == vec2(-1.0, -1.0) {
            let min = self.min;
            let max = middle_point;
            let new_points = Self::get_points_inside(&self.points, min, max);
            quarter_node = Node::new(min, max, new_points);
        } else if quarter == vec2(1.0, -1.0) {
            let min = vec2(middle_point.x, self.min.y);
            let max = vec2(self.max.x, middle_point.y);
            let new_points = Self::get_points_inside(&self.points, min, max);
            quarter_node = Node::new(min, max, new_points);
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "No quarter is not valid"));
        }

        return Ok(quarter_node);
    }
    // draw using annou
    fn draw(&self, draw: &Draw){
        let mid_points = (self.min + self.max)/2.0;
        draw.rect()
            .xy(mid_points)
            .w_h(self.max.x - self.min.x, self.max.y - self.min.y)
            .stroke_weight(1.0)
            .stroke(BLACK);
        for node in self.children.iter(){
            if let Some(n) = node {
                n.draw(draw);
            }
        }
    }
}

impl Node {
    fn get_points_inside(points: &Vec<Rc<Vec2>>, min: Vec2, max: Vec2) -> Vec<Rc<Vec2>>{
        let inside = |point: &Rc<Vec2>| point.x < max.x && point.x > min.x && point.y < min.y && point.y > min.y;
        let new_points = points.iter().map(|p| Rc::clone(p)).filter(inside).collect();
        return new_points;
    }
}
impl NodeTrait for Tree {
    fn draw(&self, draw: &Draw){
        self.node.as_ref().unwrap().draw(draw);
    }

    fn point_inside(&self, point: &Rc<Vec2>) -> bool{
        self.node.as_ref().unwrap().point_inside(point)
    }

    fn quarter(&self, point: &Rc<Vec2>) -> Vec2{
        self.node.as_ref().unwrap().quarter(point)
    }

    fn quarter_index(&self, quarter: Vec2) -> usize{
        self.node.as_ref().unwrap().quarter_index(quarter)
    }

    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error>{
        Node::index_quarter(index)
    }

    fn add_point<C>(&mut self, point: Rc<Vec2>, condition: C)
    where C: Fn(&Vec<Rc<Vec2>>) -> bool{
        self.node.as_mut().unwrap().add_point(point, condition);
    }

    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error>{
        self.node.as_ref().unwrap().add_node(quarter)
    }



}

pub trait NodeTrait {
    fn point_inside(&self, point: &Rc<Vec2>) -> bool;
    fn quarter(&self, point: &Rc<Vec2>) -> Vec2;
    fn quarter_index(&self, quarter: Vec2) -> usize;
    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error>;
    fn add_point<C>(&mut self, point: Rc<Vec2>, condition: C)
    where C: Fn(&Vec<Rc<Vec2>>) -> bool;
    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error>;
    fn draw(&self, draw: &Draw);
    
}

impl Tree {
    pub fn size(&self) -> usize{
        self.node.as_ref().unwrap().points.len()
    }

    pub fn draw_points(&self, draw: &Draw){
        for point in self.node.as_ref().unwrap().points.iter(){
            draw.ellipse().xy(point.xy()).w_h(5.0, 5.0).color(BLACK);
        }

    }
}