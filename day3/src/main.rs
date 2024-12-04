// PART 1
//use regex;
//
//fn main() {
//    let contents = std::fs::read_to_string("real.txt").unwrap();
//    let pattern = regex::Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)").unwrap();
//
//    //println!("{}", contents);
//    println!(
//        "{:?}",
//        pattern.find_iter(&contents).map(|c| c.as_str()).count() //.collect::<String>()
//    );
//    println!(
//        "{:#?}",
//        pattern
//            .find_iter(&contents)
//            .map(|m| m.as_str()[4..m.as_str().len() - 1]
//                .split(',')
//                .collect::<Vec<_>>()
//                .iter()
//                .map(|x| x.parse::<u32>().unwrap())
//                .product::<u32>())
//            .sum::<u32>()
//    );
//}
//
// PART 2
use regex;

fn search_enabled(allowed: &Vec<(bool, usize)>, position: usize) -> bool {
    // handle edges
    if position > allowed.last().unwrap().1 {
        return allowed.last().unwrap().0;
    }
    if position < allowed[0].1 {
        return true;
    }
    let mut allowed = allowed.clone();
    allowed.reverse();

    for range in allowed.iter() {
        if range.1 <= position {
            return range.0;
        }
    }
    None::<u32>.unwrap();
    false
}

fn main() {
    let contents = std::fs::read_to_string("real.txt").unwrap();
    let mul_pattern = regex::Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)").unwrap();
    let stop_pattern = regex::Regex::new(r"don't\(\)").unwrap();
    let mut stops = stop_pattern
        .find_iter(&contents)
        .map(|chunk| (false, chunk.end()))
        .collect::<Vec<_>>();
    //println!("{:#?}", stops);
    let start_pattern = regex::Regex::new(r"do\(\)").unwrap();
    let starts = start_pattern
        .find_iter(&contents)
        .map(|chunk| (true, chunk.end()))
        .collect::<Vec<_>>();
    stops.extend(starts.iter());
    stops.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
    //println!("{:#?}", stops);

    //println!("{}", contents);
    //println!(
    //    "{:?}",
    //    mul_pattern.find_iter(&contents).map(|c| c.as_str()).count() //.collect::<String>()
    //);

    println!(
        "{:#?}",
        mul_pattern
            .find_iter(&contents)
            .map(|m| (
                m.as_str()[4..m.as_str().len() - 1]
                    .split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<u32>().unwrap())
                    .product::<u32>(),
                m.start()
            ))
            .filter(|mul| search_enabled(&stops, mul.1))
            .map(|x| x.0)
            .sum::<u32>()
    );
}
