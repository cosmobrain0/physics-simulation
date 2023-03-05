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

    pub fn triangle_collision(t1: &Triangle, t2: &Triangle, eps: f32) -> bool {
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
}