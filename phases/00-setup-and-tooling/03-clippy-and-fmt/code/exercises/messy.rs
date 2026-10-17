// Practice 1: the "day one" version of src/main.rs.
// It compiles, but rustfmt and clippy have a lot to say.
//
//   cp exercises/messy.rs src/main.rs     (from this lesson's code/ folder; keep a copy of the clean one)
//   cargo fmt -p clippy-and-fmt
//   cargo clippy -p clippy-and-fmt
//
// Fix every warning WITHOUT adding #[allow(...)]. Then compare with the clean version.
fn average(scores: &Vec<f64>) -> f64 {
    let mut total = 0.0;
  for i in 0..scores.len() { total = total + scores[i]; }
    return total / scores.len() as f64;
}

fn passing(names: &Vec<String>, scores: &Vec<f64>, threshold: f64) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for i in 0..names.len() {
        if (scores[i] >= threshold) == true { out.push(names[i].clone()) }
    }
    return out;
}

fn main() {
    let names = vec!["ana".to_string(), "binh".to_string(), "chi".to_string(), "dung".to_string()];
    let scores = vec![0.91, 0.42, 0.77, 0.65];
    let avg = average(&scores);
    println!("average: {}", format!("{:.3}", avg));
    println!("passing (>= 0.6): {:?}", passing(&names, &scores, 0.6));
    let empty: Vec<f64> = vec![];
    println!("average of nothing: {}", average(&empty));
}
