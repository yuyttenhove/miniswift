
pub fn orient_2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    let s1x = ax - cx;
    let s1y = ay - cy;

    let s2x = bx - cx;
    let s2y = by - cy;

    s1x*s2y - s1y*s2x
}