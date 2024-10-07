use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, collections::HashSet, rc::Rc, vec};

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
// use trees::Vec2Weapper;
use trees::{NodeTrait, Tree};
use std::thread;

mod trees;

struct Model {
    // window: Window,
    egui: Egui,
    tree: Arc<Mutex<trees::Tree>>,
    once: bool,
    // points: Vec<Arc<Mutex<Vec2>>>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let num = 0.0;
    let mut points = vec![];

    for i in 0..10000 {
        let x = random_range(-500.0, 500.0);
        let y = random_range(-500.0, 500.0);
        let point = Mutex::new(vec2(x, y));
        points.push(Arc::new(point));
    }
    let mut tree = Arc::new(Mutex::new(Tree::new(vec2(-500.0, -500.0), vec2(500.0, 500.0))));
    for point in &points {
        tree.lock().unwrap().add_point(point.clone());
    }

    let cloned_tree = tree.clone();

    
    let model = Model {
        egui,
        tree: cloned_tree,
        once: true,
        // points,
    };
    thread::spawn(move || {
        loop {
            tree.clone().lock().unwrap().update(&points);
        }
    });
    model
}

fn update(app: &App, model: &mut Model, update: Update) {
    render_egui(&mut model.egui, &app);

    
    // println!("{}", model.tree.size());
    // bouncy_points(&mut model.points);
    // for point in &model.points {
    //     point.lock().unwrap().x += 0.15;
    //     point.lock().unwrap().y += 0.15;
    // }
    // let mut points: HashSet<Vec2ForHashSet> = HashSet::new();
    // model.tree.update(&model.points);
    // println!("{}", points.len());

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
fn render_egui(egui: &mut Egui, app: &App) {
    let egui = egui;
    // egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label(format!("FPS: {}", app.fps()));
        // ui.label("res"); // template
        // ui.add(egui::Slider::new(&mut model.num, 1.0..=40.0));
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.tree.lock().unwrap().draw(&draw);

    model.tree.lock().unwrap().draw_points(&draw);
    // draw.text(format!("FPS: {}", app.fps()).as_str())
    //     .color(BLACK)
    //     .font_size(20)
    //     .x_y(0.0, 0.0);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn bouncy_points(points: &mut Vec<Arc<Mutex<Vec2>>>) {
    for point in points {
        let mut point = point.lock().unwrap();
        if point.x > 500.0 || point.x < -500.0 {
            point.x = -point.x;
        }
        if point.y > 500.0 || point.y < -500.0 {
            point.y = -point.y;
        }
    }

}