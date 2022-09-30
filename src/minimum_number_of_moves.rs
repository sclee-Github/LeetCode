pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    seats
        .iter()
        .zip(students.iter())
        .map(|(seat, student)| (seat - student).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_number_of_moves() {
        let my_seats: Vec<i32> = vec![3, 1, 5];
        let my_students: Vec<i32> = vec![2, 7, 4];

        assert_eq!(min_moves_to_seat(my_seats, my_students), 4);
    }
}
