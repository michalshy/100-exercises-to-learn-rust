// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread::ScopedJoinHandle;

pub fn sum(v: Vec<i32>) -> i32 {
    std::thread::scope(|scope| {
        let raz: ScopedJoinHandle<i32> = scope.spawn(|| {
            v[..v.iter().len()/2].iter().sum()
        });
        let dwa: ScopedJoinHandle<i32> = scope.spawn(|| {
            v[v.iter().len()/2..].iter().sum()
        });
        raz.join().unwrap() + dwa.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
