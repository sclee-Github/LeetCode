use std::collections::HashSet;

pub fn get_number_of_good_pairs(nums: Vec<i32>) -> i32 {
    let uniq_nums: HashSet<i32> = nums.iter().cloned().collect();
    // let uniq_nums: Vec<i32> = nums.into_iter().unique().collect();
    // nums.sort();
    // nums.dedup();

    // println!("{:?}", nums.iter().map(|x| nums.iter().filter(|y| *y == x).count() as i32).collect::<Vec<i32>>());

    uniq_nums.into_iter().fold(0, |total_count, uniq_num| {
        let count = nums.iter().filter(|num| *num == &uniq_num).count() as i32;
        total_count + (count * (count - 1) / 2) as i32
    })

    /* First Solution
    let mut count_hashmap: HashMap<i32, i32> = HashMap::new();
    let mut total_count = 0;

    for num in nums {
        match count_hashmap.get(&num) {
            None => {
                count_hashmap.insert(num, 1);
                // println!("Insert counter:1 for key value {}", num);
            }

            Some(&counter) => {
                total_count += counter;
                count_hashmap.insert(num, counter + 1);
                // println!("Intert counter:{} for key value {}", counter + 1, num);
            }
        }
    }

    total_count
     */
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn number_of_good_pairs() {
        let my_vec: Vec<i32> = vec![1, 2, 3, 1, 1, 3];
        assert_eq!(get_number_of_good_pairs(my_vec), 4);
    }
}
