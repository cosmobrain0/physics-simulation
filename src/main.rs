mod triangle;
mod vector;

use flo_draw::*;
use flo_canvas::*;
use vector::vector::Vector;
use triangle::triangle::Triangle;
use std::cell::RefCell;
use std::sync::Mutex;

struct AppData {
    triangle: Triangle
}

pub fn main() {
    with_2d_graphics(|| {
        let mut triangle = Triangle::new(Vector::new(400.0, 400.0), Vector::new(600.0, 400.0), Vector::new(500.0, 600.0));
        triangle.accelerate(Vector::right());
        triangle.accelerate_rotation(0.02);

        let data = Mutex::new(RefCell::new(AppData {
            triangle
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
    data.triangle.update();
    
    ctx.clear_canvas(Color::Rgba(0.0, 0.4, 0.4, 1.0));
    ctx.canvas_height(1000.0);
    ctx.center_region(0.0, 0.0, 1000.0, 1000.0);
    
    ctx.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));
    
    data.triangle.draw(ctx);
}