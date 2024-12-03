// PART 1
//fn main() {
//    let lines = std::fs::read_to_string("real.txt")
//        .unwrap()
//        .lines()
//        .filter(|l| l.trim() != "")
//        .map(|l| {
//            l.split_whitespace()
//                .map(|x| x.parse::<i32>().unwrap())
//                .collect::<Vec<i32>>()
//        })
//        .collect::<Vec<Vec<i32>>>();
//    let mut deltas: Vec<Vec<i32>> = vec![];
//    for (l, line) in lines.iter().enumerate() {
//        deltas.push(vec![]);
//        for (i, x) in line.iter().enumerate() {
//            if i < line.len() - 1 {
//                deltas[l].push(line[i + 1] - x);
//            }
//        }
//    }
//
//    //println!("{:#?}", deltas);
//    println!(
//        "{}",
//        deltas
//            .iter()
//            .filter(|l| {
//                l.iter()
//                    .all(|d| vec![-3, -2, -1, 1, 2, 3].contains(d) && d.signum() == l[0].signum())
//            })
//            .count()
//    );
//}
//

// PART 2
fn main() {
    let lines = std::fs::read_to_string("real.txt")
        .unwrap()
        .lines()
        .filter(|l| l.trim() != "")
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut deltas: Vec<(usize, Vec<i32>)> = vec![];
    for (l, line) in lines.iter().enumerate() {
        deltas.push((l, vec![]));
        for (i, x) in line.iter().enumerate() {
            if i < line.len() - 1 {
                deltas[l].1.push(line[i + 1] - x);
            }
        }
    }

    //println!("{:#?}", deltas);
    let to_check = deltas.iter().filter(|l| {
        l.1.iter()
            .filter(|d| vec![-3, -2, -1, 1, 2, 3].contains(d) && d.signum() == l.1[0].signum())
            .count()
            < l.1.len()
    });
    //println!("{}", to_check.count());
    let mut count = 0;
    for (i, l) in to_check.clone() {
        let og = lines[*i].clone();
        for j in 0..og.len() {
            let mut foo = og[0..j].to_vec();
            foo.extend_from_slice(&og[j + 1..og.len()]);
            //println!("{}, {:?}\t{:?}", j, og, foo);
            let mut d: Vec<i32> = vec![];
            for (i, x) in foo.iter().enumerate() {
                if i < foo.len() - 1 {
                    d.push(foo[i + 1] - x);
                }
            }
            if d.iter()
                .all(|k| vec![-3, -2, -1, 1, 2, 3].contains(k) && k.signum() == d[0].signum())
            {
                count += 1;
                break;
            } else {
                println!("{:?}\t{:?}", og, d);
            }
            //println!("{:?}", og);
        }
        //println!();
    }
    println!("{}", count + lines.len() - to_check.count());
}
