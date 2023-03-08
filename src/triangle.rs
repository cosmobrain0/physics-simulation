pub mod triangle {
    use crate::collision;
    use crate::vector::vector::Vector;
    use flo_canvas::{CanvasGraphicsContext, GraphicsContext};

    #[derive(Debug)]
    pub struct TrianglePoint {
        pub position: Vector,
        velocity: Vector,
    }

    #[derive(Debug)]
    pub struct Triangle {
        pub points: [TrianglePoint; 3]
    }

    impl Triangle {
        pub fn new(p1: Vector, p2: Vector, p3: Vector) -> Self { Self { points: [
            TrianglePoint { position: p1, velocity: Vector::zero() },
            TrianglePoint { position: p2, velocity: Vector::zero() },
            TrianglePoint { position: p3, velocity: Vector::zero() }
        ]}}

        pub fn draw(&self, ctx: &mut CanvasGraphicsContext) {
            ctx.new_path();
            ctx.move_to(self.points[0].position.x, self.points[0].position.y);
            ctx.line_to(self.points[1].position.x, self.points[1].position.y);
            ctx.line_to(self.points[2].position.x, self.points[2].position.y);
            ctx.line_to(self.points[0].position.x, self.points[0].position.y);
            ctx.fill();
        }

        fn centre(&self) -> Vector { self.points.iter().fold(Vector::zero(), |acc, val| { acc + val.position }) / 3.0 }

        pub fn update(&mut self) {
            // update position and rotation
            // TODO: this asap
            // for each point, calculate angular velocity and actual velocity
            // sum them
            // apply result to all points
            let centre = self.centre();
            let (direct_velocity, angular_velocity) = self.points.iter().fold((Vector::zero(), 0.0), |acc, val| {
                let offset = val.position - centre;
                // project velocity onto offset and find remainder
                let offset_projection = val.velocity.project(&offset);
                let remainder = val.velocity - offset_projection;
                // calculate radians change based on offset length and remainder length (remainder wraps around circle with radius offset.length)
                // circumference: 2 * pi * offset.length()
                // length: remainder.length() = k * offset.length()
                // radians: k = remainder.length() / offset.length()
                // clockwise / counter-clockwise: dot(remainder, offset_rotated_90deg_clockwise) > 0 ? clockwise : counter-clockwise
                let radians = remainder.length() / offset.length() * if remainder.dot(&offset.clockwise_90deg()) > 0.0 { 1.0 } else { -1.0 };
                (acc.0 + offset_projection, acc.1 + radians)
            });
            self.translate(direct_velocity);
            self.rotate(angular_velocity);
            // panic!("Haven't written triangle updating code yet!");
        }

        pub fn accelerate(&mut self, accelerate_from: Vector, acceleration: Vector) {
            let distances: Vec<_> = self.points.iter().map(|x| { (x.position-accelerate_from).length() }).collect();
            let max_distance = distances.iter().fold(0.0, |acc, val| { acc + val });
            self.points.iter_mut().enumerate().for_each(|(i, x)| { x.velocity += acceleration * (1.0 - distances[i] / max_distance) / 2.0; });
        }

        pub fn translate(&mut self, offset: Vector) {
            self.points.iter_mut().for_each(|x| { x.position += offset; });
        }

        pub fn rotate(&mut self, offset: f32) {
            let centre = self.centre();
            self.points.iter_mut().for_each(|x| { x.position = (x.position - centre).rotate(offset) + centre });
        }

        pub fn collision_points(tri1: &Triangle, tri2: &Triangle) -> Option<Vec<Vector>> {
            collision::collision::triangle_collision(tri1, tri2)
        }
    }
}