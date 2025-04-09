use std::fs::File;
use std::io::BufRead;

fn populate_lists(path: &str, left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) {
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let digits: Vec<&str> = line.split_ascii_whitespace().collect();
        left_list.push(digits[0].parse::<i32>().unwrap());
        right_list.push(digits[1].parse::<i32>().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_total_distances() {
        let mut left_vec = Vec::<i32>::new();
        let mut right_vec = Vec::<i32>::new();
        populate_lists("./input.txt", &mut left_vec, &mut right_vec);
        left_vec.sort();
        right_vec.sort();
        let mut pairs = Vec::new();
        for (lv, rv) in left_vec.iter().zip(right_vec.iter()) {
            pairs.push((*lv, *rv));
        }
        println!("{:?}", pairs);
        let dists: Vec<i32> = pairs.iter().map(|e| (e.1 - e.0).abs()).collect();
        println!("{:?}", dists);
        assert_eq!(1941353, dists.iter().sum());
    }

    #[test]
    fn day1_part2_similarity_index() {
        use std::collections::HashMap;
        let mut left_vec = Vec::<i32>::new();
        let mut right_vec = Vec::<i32>::new();
        populate_lists("./input.txt", &mut left_vec, &mut right_vec);
        let mut scores: HashMap<i32, i32> = HashMap::new();
        for li in &left_vec {
            scores.insert(*li, 0);
            for ri in &right_vec {
                if *li == *ri {
                    scores.insert(*li, 1 + scores[li]);
                } else {
                    continue;
                };
            }
        }
        println!("{:?}", scores);
        let mut sum_of_scores = 0;
        for (k, v) in &scores {
            sum_of_scores += *k * *v;
        }
        assert_eq!(22539317, sum_of_scores);
    }
}
