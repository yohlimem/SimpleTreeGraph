use std::{cell::RefCell, rc::Rc, vec};

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use trees::{NodeTrait, Tree};

mod trees;

struct Model {
    // window: Window,
    egui: Egui,
    tree: trees::Tree,
    once: bool,
    points: Vec<Rc<RefCell<Vec2>>>,
}

fn main() {
    nannou::app(model).update(update).run();

}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let num = 0.0;
    let mut points = vec![];

    for i in 0..100 {
        let x = random_range(-500.0, 500.0);
        let y = random_range(-500.0, 500.0);
        let point = RefCell::new(vec2(x, y));
        points.push(Rc::new(point));
    }
    let mut tree = Tree::new(vec2(-500.0, -500.0), vec2(500.0, 500.0));
    for point in &points {
        tree.add_point(point.clone());
    }    
    Model {
        egui,
        tree,
        once: true,
        points,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    render_egui(&mut model.egui);
    println!("{}", model.tree.size());
    for point in &model.points {
        point.borrow_mut().x += random_range(-1.0, 1.0) * 5.0;
        point.borrow_mut().y += random_range(-1.0, 1.0) * 5.0;
    }

    model.tree.update();




    // when clicked, add a point to the tree but not when held
    // if app.mouse.buttons.left().is_down() && model.once {
    //     let mouse = app.mouse.position();
    //     model.tree.add_point(Rc::new(RefCell::new(vec2(mouse.x, mouse.y))));
    //     model.once = false;
    //     println!("up");
    // } 
    // if app.mouse.buttons.left().is_up() {
    //     model.once = true;
    // }

}
fn render_egui(egui: &mut Egui){
    let egui = egui;
    // egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        // ui.label("res"); // template
        // ui.add(egui::Slider::new(&mut model.num, 1.0..=40.0));
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.tree.draw(&draw);

    model.tree.draw_points(&draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
