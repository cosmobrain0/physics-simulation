pub mod collision {
    use crate::triangle::triangle::Triangle;
    use crate::vector::vector::Vector;

    struct TrianglePoints(Vector, Vector, Vector);

    fn det2d(t: &TrianglePoints) -> f32 {
        t.0.x * (t.1.y - t.2.y)
        + t.1.x * (t.2.y - t.0.y)
        + t.2.x * (t.0.y - t.1.y)
    }

    fn check_winding(t: &TrianglePoints) -> Result<(), ()> {
        let det = det2d(t);
        if (det < 0.0) { return Err(()); }
        Ok(())
    }

    fn boundary_collision_check(t: &TrianglePoints, eps: f32) -> bool { det2d(t) < eps }

    fn working_triangle_collision(t1: &Triangle, t2: &Triangle, eps: f32) -> bool {
        // on-boundary is assumed true
        // allow-reversed is asumed false
        // these variables are replaced with their values in this translation of the original c# code.

        // triangles must be counter-clockwise
        check_winding(&TrianglePoints(t1.points[0], t1.points[1], t1.points[2])).unwrap();
        check_winding(&TrianglePoints(t2.points[0], t2.points[1], t2.points[2])).unwrap();

        let check_edge = boundary_collision_check;
        let lp1 = t1.points;
        let lp2 = t2.points;

        for i in 0..3 {
            let j = (i+1)%3;
            if check_edge(&TrianglePoints(lp1[i], lp1[j], lp2[0]), eps) &&
                check_edge(&TrianglePoints(lp1[i], lp1[j], lp2[1]), eps) &&
                check_edge(&TrianglePoints(lp1[i], lp1[j], lp2[2]), eps) { return false; }
        }

        for i in 0..3 {
            let j = (i+1)%3;
            if check_edge(&TrianglePoints(lp2[i], lp2[j], lp1[0]), eps) &&
                check_edge(&TrianglePoints(lp2[i], lp2[j], lp1[1]), eps) &&
                check_edge(&TrianglePoints(lp2[i], lp2[j], lp1[2]), eps) { return false; }
        }

        true
    }

    fn triangle_area(p1: Vector, p2: Vector, p3: Vector) -> f32 {
        ((p2.x-p1.x)*(p3.y-p1.y) - (p3.x-p1.x)*(p2.y-p1.y)).abs()
    }

    // if a triangle is completely inside another triangle, no collision should be registered.
    fn triangle_point_collision(triangle: &Triangle, point: Vector) -> bool {
        (triangle_area(triangle.points[0], triangle.points[1], triangle.points[2]) - (
            triangle_area(triangle.points[0], triangle.points[1], point) +
            triangle_area(triangle.points[1], triangle.points[2], point) +
            triangle_area(triangle.points[2], triangle.points[0], point)
        )).abs() <= 0.001
    }

    struct Line {
        p1: Vector, p2: Vector, m: Option<f32>, c: Option<f32>
    }
    impl Line {
        pub fn new(p1: Vector, p2: Vector) -> Self {
            if p1.x == p2.x {
                // vertical line
                Self { p1, p2, m: None, c: None }
            } else {
                let m = (p2.y-p1.y)/(p2.x-p1.x);
                Self { p1, p2, m: Some(m), c: Some(p1.y-m*p1.x) }
            }
        }

        pub fn perpendicular_through_point(&self, point: Vector) -> Self {
            if self.m == Some(0.0) {
                // horizontal line
                Self { p1: point, p2: Vector::new(point.x, self.p1.y), m: None, c: None }
            } else if let None = self.m {
                // vertical line
                Self { p1: point, p2: Vector::new(self.p1.x, point.y), m: Some(0.0), c: Some(point.y) }
            } else {
                let m = self.m.unwrap();
                let c = self.c.unwrap();
                let m2 = -1.0/m;
                let c2 = point.y - m2*point.x;
                let p2x = (c2-c)/(m-m2);
                Self { p1: point, p2: Vector::new(p2x, m2*p2x+c2), m: Some(m2), c: Some(c2) }
            }
        }
    }

    fn range_overlap(range1: (f32, f32), range2: (f32, f32)) -> Option<(f32, f32)> {
        if range1.1.clamp(range2.0, range2.1) == range1.1 { Some((range2.0.max(range1.0), range1.1)) }
        else if range1.0.clamp(range2.0, range2.1) == range1.0 { Some((range1.0, range1.1.min(range2.1))) }
        else if range2.1.clamp(range1.0, range1.1) == range2.1 { Some((range1.0.max(range2.0), range2.1)) }
        else if range2.0.clamp(range1.0, range1.1) == range2.0 { Some((range2.0, range2.1.min(range1.1))) }
        else { None }
    }

    fn in_range(value: f32, boundary_one: f32, boundary_two: f32) -> bool { value >= boundary_one.min(boundary_two) && value <= boundary_two.max(boundary_one) }

    fn line_line_intersection(l1: &Line, l2: &Line) -> bool {
        // ignoring vertical lines and horizontal lines
        match (l1.m, l1.c, l2.m, l2.c) {
            (Some(m1), Some(c1), Some(m2), Some(c2)) => {
                if m1 == m2 && c1 == c2 {
                    range_overlap(
                        if l1.p1.x < l1.p2.x { (l1.p1.x, l1.p2.x) } else { (l1.p2.x, l1.p1.x) },
                        if l2.p1.x < l2.p2.x { (l2.p1.x, l2.p2.x) } else { (l2.p2.x, l2.p1.x) },
                    ) != None
                } else if m1 == m2 { false }
                else {
                    let x = (c1-c2)/(m2-m1);
                    in_range(x, l1.p1.x, l1.p2.x) && in_range(x, l2.p1.x, l2.p2.x)
                }
            },
            _ => false // TODO: this
        }
    }

    pub fn triangle_collision(t1: &Triangle, t2: &Triangle) -> bool {
        for i in 0..3 {
            let line1 = Line::new(t1.points[i], t1.points[(i+1)%3]);
            for j in 0..3 {
                let line2 = Line::new(t2.points[i], t2.points[(i+1)%3]);
                if line_line_intersection(&line1, &line2) { return true; }
            }
        }
        false
    }
}