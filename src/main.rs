fn main() {
    let program = String::from("1 + 1 + 1");
    let v: Vec<i32> = program.split('+').into_iter().map(|s| s.trim().parse().unwrap()).collect();
    assert_eq!(v, [1, 1, 1]);

    let mut sum: i32 = 0;
    for n in v.into_iter() {
        sum += n;
    }

    println!("{}", sum);
}
