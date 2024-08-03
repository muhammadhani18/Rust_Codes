use std::collections::HashMap;

struct Problems <'a> {
    arr: &'a mut Vec<i32>,
    target: i32,
}

impl <'a> Problems <'a> {
    fn new(arr: &'a mut Vec<i32>, target: i32) -> Problems <'a> { 
        Problems {
            arr,
            target,
        }
    }
    fn bubble_sort(&mut self) {
        for i in 0..self.arr.len() {
            for j in 0..self.arr.len() - i -1 {
                if self.arr[j] > self.arr[j+1] {
                    self.arr.swap(j, j+1);
                }
            }
        }
        for i in 0..self.arr.len() {
            println!("{} ", self.arr[i]);
        }
    }
    fn two_sum(&mut self) {
        for i in 0..self.arr.len() - 1 {
            if self.arr[i] + self.arr[i+1] == self.target {
                println!("{} + {} = {}", self.arr[i], self.arr[i+1], self.target);
            }
        }   
    }
}


fn main() {
    let mut arr: Vec<i32> = vec![5,1,4,2,0];
    let target:i32 = 5;


    let mut p= Problems::new(&mut arr, target);

    p.two_sum();
    p.bubble_sort();
}
