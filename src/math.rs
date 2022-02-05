pub fn lininter((x1, y1): (f64, f64), (x2, y2): (f64, f64), x3: f64) -> f64 {
    y1 + (x3 - x1) * (y2 - y1) / (x2 - x1)
}

pub fn linspace(target_vector: &mut Vec<f64>, x0: f64, x1: f64, points_per_unit: f64) {
    if !target_vector.is_empty() {
        target_vector.clear()
    }
    target_vector.push(x0);
    let mut i = 1;
    while target_vector[i - 1] < x1 {
        target_vector.push(x0 + i as f64 / points_per_unit);
        i += 1;
    }
}

pub fn linspace2(x0: f64, x1: f64, n: i64) -> Vec<f64> {
    let mut vector: Vec<f64> = Vec::new();
    let dx = (x1 - x0) / n as f64;
    let mut i = x0;

    while i < x1 {
        vector.push(i);
        i += dx;
    }
    vector
}
