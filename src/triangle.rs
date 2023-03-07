pub mod triangle {
    use crate::collision;
    use crate::vector::vector::Vector;
    use flo_canvas::{CanvasGraphicsContext, GraphicsContext};

    #[derive(Debug)]
    pub struct Triangle {
        pub points: [Vector; 3],
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

        fn triangle_cross2(tri1: &Triangle, tri2: &Triangle) -> bool {
            let da = tri1.points[0] - tri2.points[1];
            let db = tri1.points[1] - tri2.points[1];
            let dc = tri1.points[2] - tri2.points[1];
            let dx21 = tri2.points[2].x - tri2.points[1].x;
            let dy12 = tri2.points[1].y - tri2.points[2].y;
            let D = dy12 * (tri2.points[0].x - tri2.points[2].x) + dx21 * (tri2.points[0].y - tri2.points[2].y);
            let sa = dy12 * da.x + dx21 * da.y;
            let sb = dy12 * db.x + dx21 * db.y;
            let sc = dy12 * dc.x + dx21 * dc.y;
            let ta = (tri2.points[2].y - tri2.points[0].y) * da.x + (tri2.points[0].x - tri2.points[2].x) * da.y;
            let tb = (tri2.points[2].y - tri2.points[0].y) * db.x + (tri2.points[0].x - tri2.points[2].x) * db.y;
            let tc = (tri2.points[2].y - tri2.points[0].y) * dc.x + (tri2.points[0].x - tri2.points[2].x) * dc.y;

            if D < 0.0 { return (sa >= 0.0 && sb >= 0.0 && sc >= 0.0) || (ta >= 0.0 && tb >= 0.0 && tc >= 0.0) || (sa+ta <= D && sb+tb <= D && sc+tc <= D); }

            (sa <= 0.0 && sb <= 0.0 && sc <= 0.0) || (ta <= 0.0 && tb <= 0.0 && tc <= 0.0) || (sa+ta >= D && sb+tb >= D && sc+tc >= D)
        }

        pub fn collision_points(tri1: &Triangle, tri2: &Triangle) -> Option<Vec<Vector>> {
            collision::collision::triangle_collision(tri1, tri2)
        }
    }
}