//Time:      7  15   30
//Distance:  9  40  200

//Time:        59     70     78     78
//Distance:   430   1218   1213   1276

fn simulate(time : i64, distance : i64) -> i64 {
    //println!("time {} distance {}", time, distance);
    let mut winners : i64 = 0;
    for i in 0 .. time {
        let travel = i * (time - i);
        let winner = travel > distance;
        if winner {
            winners += 1;
        }
        //println!("speed {} distance {} winner {}", i, travel, winner);
    }
    //println!("  winners {}", winners);
    return winners;
}

fn main() {
    //let result : i64 = simulate(7,9) * simulate(15,40) * simulate(30,200);
    let result : i64 = simulate(59, 430) * simulate(70, 1218) * simulate(78, 1213) * simulate(78, 1276);

    //part b
    //let result : i64 = simulate(59707878, 430121812131276);

    println!("result {}", result);
}
