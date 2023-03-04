pub mod triangle {
    use crate::vector::vector::Vector;
    use flo_canvas::{CanvasGraphicsContext, GraphicsContext};

    #[derive(Debug)]
    pub struct Triangle {
        points: [Vector; 3],
        centre: Vector,
        velocity: Vector,
        rotation: f32, // radians
        angular_velocity: f32,
    }

    impl Triangle {
        pub fn new(p1: Vector, p2: Vector, p3: Vector) -> Self { Self { points: [p1, p2, p3], centre: (p1+p2+p3)/3., velocity: Vector::zero(), rotation: 0.0, angular_velocity: 0.0 }}
        pub fn draw(&self, ctx: &mut CanvasGraphicsContext) {
            ctx.new_path();
            ctx.move_to(self.points[0].x, self.points[0].y);
            ctx.line_to(self.points[1].x, self.points[1].y);
            ctx.line_to(self.points[2].x, self.points[2].y);
            ctx.line_to(self.points[0].x, self.points[0].y);
            ctx.fill();
        }

        pub fn update(&mut self) {
            // update position and rotation
            self.translate(self.velocity);
            self.rotate(self.angular_velocity);
        }

        pub fn accelerate(&mut self, acceleration: Vector) {
            self.velocity += acceleration;
        }

        pub fn accelerate_rotation(&mut self, acceleration: f32) {
            self.angular_velocity += acceleration;
        }

        fn translate(&mut self, offset: Vector) {
            self.centre += offset;
            self.points.iter_mut().for_each(|x| { *x += offset; });
        }

        fn rotate(&mut self, delta: f32) {
            self.points.iter_mut().for_each(|x| {
                *x = self.centre + (*x-self.centre).rotate(delta);
            })
        }
    }
}