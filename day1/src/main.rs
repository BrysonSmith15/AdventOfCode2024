// PT 1
//fn main() -> std::io::Result<()> {
//    let input = std::fs::read_to_string("real.txt")?;
//    let lines = input
//        .split('\n')
//        .filter(|x| x.trim() != "")
//        .map(|x| {
//            x.split("   ")
//                .map(|y| y.parse::<u32>().unwrap())
//                .collect::<Vec<u32>>()
//        })
//        .collect::<Vec<Vec<u32>>>();
//    let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = lines.iter().map(|l| (l[0], l[1])).unzip();
//    col1.sort();
//    col2.sort();
//    println!("{:?}", col1);
//    println!("{:?}", col2);
//    let mut sum = 0;
//    while let (Some(i), Some(j)) = (col1.pop(), col2.pop()) {
//        sum += i.abs_diff(j)
//    }
//    println!("{}", sum);
//    Ok(())
//}

// PT 2
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("real.txt")?;
    let lines = input
        .split('\n')
        .filter(|x| x.trim() != "")
        .map(|x| {
            x.split("   ")
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = lines.iter().map(|l| (l[0], l[1])).unzip();
    let mut counts: Vec<(u32, u32)> = Vec::from_iter(
        col1.iter()
            .map(|x| (*x, col2.iter().filter(|y| y == &x).count() as u32)),
    );
    println!("{:?}", counts);
    let mut out = 0;
    for (k, v) in counts.iter() {
        out += k * v;
    }
    println!("{}", out);
    Ok(())
}
