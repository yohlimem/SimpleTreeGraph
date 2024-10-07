use geom::point;
use nannou::{glam::Vec2Swizzles, prelude::*};
use std::borrow::Borrow;
use std::hash::{self, Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::{borrow::BorrowMut, cell::RefCell, collections::HashSet, rc::Rc};

// TODO: MAKE UPDATE TREE!
pub struct Tree {
    pub node: Option<Node>,
}

pub struct Node {
    min: Vec2,
    max: Vec2,
    points: Vec<Arc<Mutex<Vec2>>>,
    children: Box<[Option<Node>; 4]>,
    is_leaf: bool,
    for_remove: bool,
}

impl Tree {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Tree {
            node: Some(Node::new(min, max, Vec::new())),
            // leaves: Vec::new(),
        }
    }
}

// #[derive(Debug)]
// pub struct Vec2Weapper(Rc<RefCell<Vec2>>);
// impl Hash for Vec2Weapper {
//     fn hash<H: hash::Hasher>(&self, state: &mut H) {
//         self.0.borrow().x.to_bits().hash(state);
//         self.0.borrow().y.to_bits().hash(state);
//     }
// }

// impl PartialEq for Vec2Weapper {
//     fn eq(&self, other: &Self) -> bool {
//         self.0.borrow().x == other.0.borrow().x && self.0.borrow().y == other.0.borrow().y
//     }
// }
// impl Eq for Vec2Weapper {}

impl Node {
    pub fn new(min: Vec2, max: Vec2, points: Vec<Arc<Mutex<Vec2>>>) -> Self {
        Node {
            min,
            max,
            points,
            children: Box::new([None, None, None, None]),
            for_remove: false,
            is_leaf: true,
        }
    }
}

impl NodeTrait for Node {
    fn point_inside(&self, point: &Vec2) -> bool {
        // let condition_x_max = point.x < self.max.x;
        // let condition_x_min = point.x > self.min.x;
        // let condition_y_max = point.y < self.max.y;
        // let condition_y_min = point.y > self.min.y;

        // println!("condition_x_max: {}", condition_x_max);
        // println!("condition_x_min: {}", condition_x_min);
        // println!("condition_y_max: {}", condition_y_max);
        // println!("condition_y_min: {}", condition_y_min);
        // println!("condition_y_min: {}", point.x < self.max.x
        //                                 && point.x > self.min.x
        //                                 && point.y < self.max.y
        //                                 && point.y > self.min.y);
        point.x < self.max.x && point.x > self.min.x && point.y < self.max.y && point.y > self.min.y
    }
    fn quarter(&self, point: &Arc<Mutex<Vec2>>) -> Vec2 {
        let middle_point = (self.min + self.max) / 2.0;

        let direction = (point.lock().unwrap().borrow().xy() - middle_point).normalize();

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

    fn add_point(&mut self, point: Arc<Mutex<Vec2>>) {

        // println!("{:?}", self.points);
        // self.points.push(point.clone());
        if !self.point_inside(&point.lock().unwrap()) {
            return;
        }

        // if self.points.contains(&point) {
        //     println!("point already in tree");
        //     // return;
        // }
        // let quarter = self.quarter(&point);

        // let mut point_count_in_quarter = self.how_many_points_in_quarter(quarter);

        self.points.push(point.clone());
        let mut currect_node = self;
        // println!("point count in quarter: {:?}", point_count_in_quarter);
        // println!("point count: {:?}", self.points.len());
        // TODO: OPTIMIZE!!! AND FIX
        while currect_node.point_inside(&point.lock().unwrap()) {
            let quarter = currect_node.quarter(&point);
            if currect_node.how_many_points_in_quarter(quarter) < 2 {
                break;
            }
            let index = currect_node.quarter_index(quarter);

            if currect_node.children[index].is_none() {
                // println!("adding new node");
                let mut new_node = currect_node.add_node(quarter).expect("no quarter found");
                new_node.points.push(point.clone());
                
                currect_node.children[index] = Some(new_node);
                
                currect_node = currect_node.children[index].as_mut().unwrap();
                
                continue;   
            }
            currect_node.children[index].as_mut().unwrap().points.push(point.clone());
            currect_node = currect_node.children[index].as_mut().unwrap();


            // println!("{point_count_in_quarter}")

            // break;
        }
        // if point_count_in_quarter >= 2 {
        //     // println!("point count: {:?}", self.points);
        //     if self.children[index].is_none() {
        //         let mut new_node = self.add_node(quarter).expect("no quarter found");
        //         new_node.add_point(Rc::clone(&point));

        //         new_node.points.push(point);
        //         // new_node.is_leaf = true;

        //         self.children[index] = Some(new_node);
        //         return;
        //     }
        //     self.children[index]
        //         .as_mut()
        //         .unwrap()
        //         .add_point(Rc::clone(&point));
        // }
    }

    fn remove_point(&mut self, point: Arc<Mutex<Vec2>>) {
        // let quarter = self.quarter(&point);
        // let index = self.quarter_index(quarter);

        // if Self::value_exists_in_vec(&self.points, &point.lock().unwrap()) {
        //     self.points.retain(|x| x.lock().unwrap() != &point.lock().unwrap());
        //     self.children[index].as_mut().unwrap().remove_point(point);
        // }
    }
    /// converts the quarter to an index
    fn quarter_index(&self, quarter: Vec2) -> usize {
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
    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error> {
        // index = 0 -> top left
        // index = 1 -> top right
        // index = 2 -> bottom left
        // index = 3 -> bottom right
        let mut quarter = Vec2::new(0.0, 0.0);

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
        if quarter == vec2(-1.0, 1.0) {
            let min = vec2(self.min.x, middle_point.y);
            let max = vec2(middle_point.x, self.max.y);
            let new_points = Self::get_points_inside(&self.points, &min, &max);
            quarter_node = Node::new(min, max, new_points)
        }
        else if quarter == vec2(1.0, 1.0) {
            let min = middle_point;
            let max = self.max;
            let new_points = Self::get_points_inside(&self.points, &min, &max);
            quarter_node = Node::new(min, max, new_points)
        }
        else if quarter == vec2(-1.0, -1.0) {
            let min = self.min;
            let max = middle_point;
            let new_points = Self::get_points_inside(&self.points, &min, &max);
            quarter_node = Node::new(min, max, new_points)
        }
        else if quarter == vec2(1.0, -1.0) {
            let min = vec2(middle_point.x, self.min.y);
            let max = vec2(self.max.x, middle_point.y);
            let new_points = Self::get_points_inside(&self.points, &min, &max);
            quarter_node = Node::new(min, max, new_points)
        }
        else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No quarter is not valid",
            ));
        }

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
        for node in self.children.iter() {
            if let Some(n) = node {
                n.draw(draw);
            }
        }
    }

    // fn update(&mut self) {
    // let mut for_remove = vec![];
    // // let mut left_points = vec![];

    // for point in 0..self.points.len() {
    //     let temp_vec = Vec2ForHashSet(self.points[point].clone());
    //     if !self.point_inside(&self.points[point].borrow()) {
    //         left_points.insert(temp_vec);
    //         for_remove.push(point);
    //     }
    // }
    // if for_remove.len() > 0 {
    //     self.for_remove = true;
    // }

    // for node in 0..self.children.len() {
    //     if let Some(n) = &mut self.children[node] {
    //         n.update(left_points);
    //         if n.for_remove {
    //             self.children[node] = None;
    //         }
    //     }
    // }
    // }
}

impl Node {
    fn get_points_inside(
        points: &Vec<Arc<Mutex<Vec2>>>,
        min: &Vec2,
        max: &Vec2,
    ) -> Vec<Arc<Mutex<Vec2>>> {
        let inside = |point: &&Arc<Mutex<Vec2>>| {
            point.lock().unwrap().borrow().x < max.x
                && point.lock().unwrap().borrow().x > min.x
                && point.lock().unwrap().borrow().y < min.y
                && point.lock().unwrap().borrow().y > min.y
        };
        let new_points = points.iter().filter(inside).map(|p| Arc::clone(p)).collect();
        return new_points;
    }

    fn how_many_points_in_quarter(&self, quarter: Vec2) -> usize {
        let mut points = 0;
        for point in self.points.iter() {
            if !self.point_inside(&point.lock().unwrap()) {
                continue;
            }
            if self.is_inside_quarter(point, quarter) {
                points += 1;
            }
        }
        points
    }

    fn is_inside_quarter(&self, point: &Arc<Mutex<Vec2>>, quarter: Vec2) -> bool {
        let middle_point = (self.min + self.max) / 2.0;

        let mut max = Vec2::new(0.0, 0.0);
        let mut min = Vec2::new(0.0, 0.0);

        if quarter.x > 0.0 {
            max.x = self.max.x;
            min.x = middle_point.x;
        } else {
            max.x = middle_point.x;
            min.x = self.min.x;
        }

        if quarter.y > 0.0 {
            max.y = self.max.y;
            min.y = middle_point.y;
        } else {
            max.y = middle_point.y;
            min.y = self.min.y;
        }

        point.lock().unwrap().borrow().x < max.x
            && point.lock().unwrap().borrow().x > min.x
            && point.lock().unwrap().borrow().y < max.y
            && point.lock().unwrap().borrow().y > min.y
    }

    fn value_exists_in_vec(vec: &Vec<Arc<Mutex<Vec2>>>, target: &Vec2) -> bool {
        for item in vec {
            if let Ok(locked_item) = item.lock() {
                if *locked_item == *target {
                    return true;
                }
            }
        }
        false
    }
}
impl NodeTrait for Tree {
    fn draw(&self, draw: &Draw) {
        self.node.as_ref().unwrap().draw(draw);
    }

    fn point_inside(&self, point: &Vec2) -> bool {
        self.node.as_ref().unwrap().point_inside(point)
    }

    fn quarter(&self, point: &Arc<Mutex<Vec2>>) -> Vec2 {
        self.node.as_ref().unwrap().quarter(point)
    }

    fn quarter_index(&self, quarter: Vec2) -> usize {
        self.node.as_ref().unwrap().quarter_index(quarter)
    }

    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error> {
        Node::index_quarter(index)
    }

    fn add_point(&mut self, point: Arc<Mutex<Vec2>>)
    /*, condition: C)
    where C: Fn(&Vec<Arc<Mutex<Vec2>>>) -> bool*/
    {
        self.node.as_mut().unwrap().add_point(point);
    }

    fn remove_point(&mut self, point: Arc<Mutex<Vec2>>) {
        self.node.as_mut().unwrap().remove_point(point);
    }

    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error> {
        self.node.as_ref().unwrap().add_node(quarter)
    }
}

pub trait NodeTrait {
    fn point_inside(&self, point: &Vec2) -> bool;
    fn quarter(&self, point: &Arc<Mutex<Vec2>>) -> Vec2;
    fn quarter_index(&self, quarter: Vec2) -> usize;
    fn index_quarter(index: u8) -> Result<Vec2, std::io::Error>;
    fn add_point(&mut self, point: Arc<Mutex<Vec2>> /*, condition: C*/);
    fn remove_point(&mut self, point: Arc<Mutex<Vec2>>);
    // where C: Fn(&Vec<Arc<Mutex<Vec2>>>) -> bool;
    fn add_node(&self, quarter: Vec2) -> Result<Node, std::io::Error>;
    fn draw(&self, draw: &Draw);
    // fn update(&mut self);
}

impl Tree {
    pub fn size(&self) -> usize {
        self.node.as_ref().unwrap().points.len()
    }

    pub fn draw_points(&self, draw: &Draw) {
        for point in self.node.as_ref().unwrap().points.iter() {
            let poin_draw = point.lock().unwrap();
            draw.ellipse().xy(poin_draw.borrow().xy()).w_h(5.0, 5.0).color(BLACK);
        }
    }

    pub fn update(&mut self, points: &Vec<Arc<Mutex<Vec2>>>) {
        self.node.as_mut().unwrap().children = Box::new([None, None, None, None]);

        self.node.as_mut().unwrap().points = Vec::new();

        for point in points.iter() {
            self.node.as_mut().unwrap().add_point(Arc::clone(point));
        }
    }
}
