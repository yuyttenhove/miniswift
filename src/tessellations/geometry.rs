mod vertex_2d;
mod triangle_2d;

pub(super) use vertex_2d::Vertex2D;
pub(super) use triangle_2d::Triangle2D;



/// Returns a positive value when the triangle formed by (ax, ay), (bx, by) and (cx, cy) is
/// positively oriented.
pub(super) fn orient_2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    let s1x = ax - cx;
    let s1y = ay - cy;

    let s2x = bx - cx;
    let s2y = by - cy;

    s1x*s2y - s1y*s2x
}


/// Returns a negative value when (dx, dy) lies in the circumscribed sphere around the triangle
/// formed by (ax, ay), (bx, by) and (cx, cy), provided that this triangle is positively oriented,
/// a positive value when the point d lies outside and zero when the point d lies on this
/// circumscribed sphere.
pub(super) fn in_circle_2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, dx: f64, dy: f64) -> f64 {
    let s1x = bx - ax;
    let s1y = by - ay;
    let s2x = cx - ax;
    let s2y = cy - ay;
    let s3x = dx - ax;
    let s3y = dy - ay;
    let (mut tmp1, mut tmp2, mut result): (f64, f64, f64);

    tmp1 = s2x*s2x +s2y*s2y;
    tmp2 = s3x*s3x +s3y*s3y;
    result = s1x * (s2y * tmp2 - s3y*tmp1);

    tmp1 = s1x*s1x +s1y*s1y;
    tmp2 = s3x*s3x +s3y*s3y;
    result -= s2x * (s1y * tmp2 - s3y*tmp1);

    tmp1 = s1x*s1x +s1y*s1y;
    tmp2 = s2x*s2x +s2y*s2y;
    result += s3x * (s1y * tmp2 - s2y*tmp1);

    result
}


pub(super) fn circumcenter_2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> Vertex2D {
    let bxn = bx - ax;
    let byn = by - ay;
    let cxn = cx - ax;
    let cyn = cy - ay;

    let tmp = 2.*(bxn * cyn - byn * cxn);
    let s3x = (cyn *(bxn * bxn + byn * byn) - byn *(cxn * cxn + cyn * cyn)) / tmp;
    let s3y = (bxn *(cxn * cxn + cyn * cyn) - cxn *(bxn * bxn + byn * byn)) / tmp;

    Vertex2D{x: s3x+ax, y: s3y+ay}
}

pub(super) fn circumradius_2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    (Vertex2D{x: ax, y: ay} - circumcenter_2d(ax, ay, bx, by, cx, cy)).norm()
}


pub(super) fn oriented_volume_2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    (ax*by + bx*cy + cx*ay - bx*ay - cx*by - ax*cy) / 2.
}


fn centroid_2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> Vertex2D {
    Vertex2D{x: (ax + bx + cx) / 3., y: (ay + by + cy) / 3.}
}