// interpolates value of f(x3) with y1 = f(x1) and y2 = f(x2) assuming that f(x) behaves linearly on (x1, x2)
pub fn linerp((x1, y1): (f64, f64), (x2, y2): (f64, f64), x3: f64) -> f64 {
    assert!(
        x2 != x1,
        "Tried to interpolate between x1 = {x1} and x2 = {x2}"
    );
    if x3 > x2 || x3 < x1 {
        println!("Tried to interpolate outside [{x1}, {x2}], x3 = {x3}");
    }
    y1 + (x3 - x1) * (y2 - y1) / (x2 - x1)
}
pub fn linerp_from_sample_rate(
    old_y: Vec<f64>,
    old_sample_rate: u32,
    new_sample_rate: u32,
) -> Vec<f64> {
    assert!(
        old_sample_rate <= new_sample_rate,
        "linerp_from_sample_rate works only if old_sample_rate ({old_sample_rate})
         is less or equal than new_sample_rate ({new_sample_rate})"
    );
    let mut new_y: Vec<f64> = Vec::new();
    let mut k: i64 = 0;
    for i in 1..old_y.len() {
        while (k * (old_sample_rate as i64)) < (i as i64 * (new_sample_rate as i64)) {
            new_y.push(linerp(
                ((i - 1) as f64 / old_sample_rate as f64, old_y[i - 1]),
                (i as f64 / old_sample_rate as f64, old_y[i]),
                k as f64 / new_sample_rate as f64,
            ));
            k += 1;
        }
    }
    new_y
}

// creates a vector that contains points spaced evenly with interval 1 / points per unit
pub fn _linspace(x0: f64, x1: f64, points_per_unit: f64) -> Vec<f64> {
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
