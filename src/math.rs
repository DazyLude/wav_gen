// interpolates value of f(x3) with y1 = f(x1) and y2 = f(x2) assuming that f(x) behaves linearly on (x1, x2)
pub fn linerp((x1, y1): (f64, f64), (x2, y2): (f64, f64), x3: f64) -> f64 {
    assert!(
        x2 != x1,
        "Tried to interpolate between x1 = {x1} and x2 = {x2}"
    );
    y1 + (x3 - x1) * (y2 - y1) / (x2 - x1)
}
// linearly interpolates vector of points for desired values spaced with desired frequency and returns desired_y
pub fn linerp_vector_from_freq(xy_old: Vec<(f64, f64)>, desired_freq: f64) -> Vec<f64> {
    let (x, y): (Vec<f64>, Vec<f64>) = xy_old.into_iter().unzip();
    let desired_x = linspace(x[0], *x.last().unwrap_or(&0.), desired_freq);

    let mut i = 0;
    let mut k = 0;
    let mut desired_y: Vec<f64> = Vec::with_capacity(desired_x.len());

    while i < desired_x.len() && k < (x.len()) {
        if desired_x[i] > x[k + 1] {
            k = k + 1;
        }
        desired_y.push(linerp((x[k], y[k]), (x[k + 1], y[k + 1]), desired_x[i]));
        i += 1;
    }
    desired_y
}

// creates a vector that contains points, spaced evenly with interval 1 / points per unit
pub fn linspace(x0: f64, x1: f64, points_per_unit: f64) -> Vec<f64> {
    assert!(
        x0 < x1,
        "Tried to create a linspace with x0 = {x0} and x1 = {x1}"
    );
    assert!(
        points_per_unit > 0.,
        "Tried to create linspace with {points_per_unit} points per unit"
    );

    let mut target_vector: Vec<f64> = Vec::new();
    target_vector.push(x0);
    let mut i = 1;
    while target_vector[i - 1] < x1 {
        target_vector.push(x0 + i as f64 / points_per_unit);
        i += 1;
    }
    target_vector.pop();
    target_vector
}
// creates a vector that contains N evenly spaced points between [x0 and x1]
pub fn linspace_from_n(x0: f64, x1: f64, n: i64) -> Vec<f64> {
    assert!(
        x1 > x0,
        "Tried to create a linspace with x0 = {x0} and x1 = {x1}"
    );
    assert!(n > 1, "Tried to create linspace with n = {n} points");

    let mut vector: Vec<f64> = Vec::with_capacity(n as usize);
    let nf = (n - 1) as f64;
    for i in 0..n {
        vector.push(x0 + (x1 - x0) * i as f64 / nf);
    }
    vector
}
