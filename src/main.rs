fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main(){
    println!("12 + 1 is {}", sum(12,1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(345+5, sum(345, 5));
    }
}