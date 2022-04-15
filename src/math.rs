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
    let mut new_y: Vec<f64> = Vec::new();
    let mut k: i64 = 0;

    match old_sample_rate.cmp(&new_sample_rate) {
        std::cmp::Ordering::Less => {
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
        }
        std::cmp::Ordering::Equal => return old_y,
        std::cmp::Ordering::Greater => {
            println!("trying to linerp data with sample rate = {old_sample_rate} to a smaller sample rate = {new_sample_rate}.");
            todo!("this is not implemented yet");
        }
    }
    new_y
}

// creates a vector that contains N evenly spaced points between [x0 and x1]
pub fn linspace_from_n(x0: f64, x1: f64, n: usize) -> Vec<f64> {
    assert!(
        x1 > x0,
        "Tried to create a linspace with x0 = {x0} and x1 = {x1}"
    );

    let mut vector: Vec<f64> = Vec::with_capacity(n as usize);
    let nf = (n - 1) as f64;
    for i in 0..n {
        vector.push(x0 + (x1 - x0) * i as f64 / nf);
    }
    vector
}
