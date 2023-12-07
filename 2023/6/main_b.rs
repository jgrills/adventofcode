//Time:      7  15   30
//Distance:  9  40  200

//Time:        59     70     78     78
//Distance:   430   1218   1213   1276

fn simulate(time : f64, distance : f64) -> i64 {
    //println!("time {} distance {}", time, distance);

    // d = i * (t - i)
    // d = it - i^2
    // i^2 - ti + d = 0
    let a = 1.0;
    let b = -time;
    let c = distance;
    let x0 = (-b + ((b*b) - (4.0 * a *c)).sqrt()) / (2.0 * a);
    //println!("  x0 {}", x0);
    let x1 = (-b - ((b*b) - (4.0 * a *c)).sqrt()) / (2.0 * a);
    //println!("  x1 {}", x1);
    let m0 = x0.min(x1).ceil();
    //println!("  m0 {}", m0);
    let m1 = x0.max(x1).floor();
    //println!("  m1 {}", m1);
    let winners = ((m1 - m0) + 1.0) as i64;
    //println!("  winnersq {}", winners);
    return winners;
}

fn main() {
    // let result : i64 = simulate(71530.0, 940200.0);
    let result : i64 = simulate(59707878.0, 430121812131276.0);
    println!("result {}", result);
}
