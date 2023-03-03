mod triangle;
mod vector;

use flo_draw::*;
use flo_canvas::*;
use vector::vector::Vector;
use triangle::triangle::Triangle;

pub fn main() {
    with_2d_graphics(|| {
        let triangle = Triangle::new(Vector::new(200.0, 200.0), Vector::new(800.0, 200.0), Vector::new(500.0, 800.0));

        let render = |ctx: &mut CanvasGraphicsContext| {
            render_loop(ctx, &triangle);
        };
        
        let canvas = create_canvas_window("Hello, triangle");
        loop {
            canvas.draw(render);
            std::thread::sleep(std::time::Duration::from_millis(1000/30));
        }
    });
}

fn render_loop(ctx: &mut CanvasGraphicsContext, triangle: &Triangle) {
    ctx.clear_canvas(Color::Rgba(0.0, 0.4, 0.4, 1.0));
    ctx.canvas_height(1000.0);
    ctx.center_region(0.0, 0.0, 1000.0, 1000.0);

    ctx.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));

    triangle.draw(ctx);
}