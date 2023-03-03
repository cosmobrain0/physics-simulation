pub mod triangle {
    use crate::vector::vector::Vector;
    use flo_canvas::{CanvasGraphicsContext, GraphicsContext};

    #[derive(Debug)]
    pub struct Triangle {
        points: [Vector; 3],
        centre: Vector,
        velocity: Vector,
    }

    impl Triangle {
        pub fn new(p1: Vector, p2: Vector, p3: Vector) -> Self { Self { points: [p1, p2, p3], centre: (p1+p2+p3)/3., velocity: Vector::zero() }}
        pub fn draw(&self, ctx: &mut CanvasGraphicsContext) {
            ctx.new_path();
            ctx.move_to(200.0, 200.0);
            ctx.line_to(800.0, 200.0);
            ctx.line_to(500.0, 800.0);
            ctx.line_to(200.0, 200.0);
            ctx.fill();
        }
    }
}