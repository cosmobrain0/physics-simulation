use flo_draw::*;
use flo_canvas::*;



pub fn main() {
    with_2d_graphics(|| {
        let canvas = create_canvas_window("Hello, triangle");
        loop {
            canvas.draw(render_loop);
            std::thread::sleep(std::time::Duration::from_millis(1000/60));
        }
    });
}

fn render_loop(ctx: &mut CanvasGraphicsContext) {
    ctx.clear_canvas(Color::Rgba(0.0, 0.4, 0.4, 1.0));
    ctx.canvas_height(1000.0);
    ctx.center_region(0.0, 0.0, 1000.0, 1000.0);

    ctx.new_path();
    ctx.move_to(200.0, 200.0);
    ctx.line_to(800.0, 200.0);
    ctx.line_to(500.0, 800.0);
    ctx.line_to(200.0, 200.0);

    ctx.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));
    ctx.fill();
}