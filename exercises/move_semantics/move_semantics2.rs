// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.


struct Solution;

impl Solution {
    // solution 1: Make a clone of vec0 and pass that as an argument to fill_vec_1
    fn solution_1() {
        let vec0 = Vec::new();
    
        let mut vec1 = Self::fill_vec_1(vec0.clone()); // Make another version of vec0
    
        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    
        vec1.push(88);
    
        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }

    fn fill_vec_1(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec.clone();

        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    }

    // solution 2: Simply borrowing vec0 and cloning the data inside
    fn solution_2() {
        let vec0 = Vec::new();

        let mut vec1 = Self::fill_vec_2(&vec0); 

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

        vec1.push(88);

        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }

    fn fill_vec_2(vec: &Vec<i32>) -> Vec<i32> {
        let mut vec = vec.clone();

        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    }

    // solution 3: Mutably borrowing vec0 and changing it
    fn solution_3() {
        let mut vec0 = Vec::new();

        let mut vec1 = Self::fill_vec_3(&mut vec0); // Mutably borrow vec0
    
        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    
        vec1.push(88);
    
        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }
    
    fn fill_vec_3(vec: &mut Vec<i32>) -> Vec<i32> {
        vec.push(22);
        vec.push(44);
        vec.push(66);
    
        vec.to_vec()
    }
}

fn main() {
    Solution::solution_1();
    Solution::solution_2();
    Solution::solution_3();
}
