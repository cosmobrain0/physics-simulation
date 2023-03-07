mod collision;
mod triangle;
mod vector;

use flo_draw::*;
use flo_canvas::*;
use vector::vector::Vector;
use triangle::triangle::Triangle;
use std::cell::RefCell;
use std::sync::Mutex;

struct AppData {
    triangleA: Triangle,
    triangleB: Triangle,
}

pub fn main() {
    with_2d_graphics(|| {
        let mut triangleA = Triangle::new(Vector::new(400.0, 400.0), Vector::new(600.0, 400.0), Vector::new(500.0, 600.0));
        let mut triangleB = Triangle::new(Vector::new(400.0, 400.0), Vector::new(600.0, 400.0), Vector::new(500.0, 600.0));
        triangleB.accelerate(Vector::right());
        triangleB.accelerate_rotation(0.02);

        let data = Mutex::new(RefCell::new(AppData {
            triangleA, triangleB
        }));
        let render = |ctx: &mut CanvasGraphicsContext| {
            render_loop(ctx, &data);
        };
        
        let canvas = create_canvas_window("Hello, triangle");
        loop {
            canvas.draw(render);
            std::thread::sleep(std::time::Duration::from_millis(1000/60));
        }
    });
}

fn render_loop(ctx: &mut CanvasGraphicsContext, data: &Mutex<RefCell<AppData>>) {
    let data_ref = data.lock().unwrap();
    let mut data = data_ref.borrow_mut();
    data.triangleA.update();
    data.triangleB.update();
    
    ctx.clear_canvas(Color::Rgba(0.0, 0.4, 0.4, 1.0));
    ctx.canvas_height(1000.0);
    ctx.center_region(0.0, 0.0, 1000.0, 1000.0);

    ctx.fill_color(Color::Rgba(0.3, 0.3, 0.3, 0.4));
    data.triangleA.draw(ctx);
    data.triangleB.draw(ctx);

    let collision_points = Triangle::collision_points(&data.triangleA, &data.triangleB);
    match collision_points {
        None => println!("No Collision!"),
        Some(points) => {
            ctx.fill_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
            for point in points.iter() {
                ctx.circle(point.x, point.y, 10.0);
            }
            ctx.fill();
        }
    }
}