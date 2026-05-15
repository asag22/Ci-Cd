fn main() {
    println!("Hello, world!");
    println!("2 + 2 = {}", add(2, 2));
    println!("100 - 34 = {}", sub(100, 33));
}

fn add(a: isize, b: isize) -> isize {
    a + b
}

fn sub(a: isize, b: isize) -> isize {
    a - b
}

#[cfg(test)]
mod testy {
    use crate::add;
    use crate::sub;

    #[test]
    fn sprawdzenie_add(){
        assert_eq!(4, add(2, 2));
        assert_eq!(54, add(50, 4));
    }
    
    #[test]
    fn sprawdzenie_sub(){
        assert_eq!(0, sub(2, 2));
        assert_eq!(46, sub(50, 4));
        assert_eq!(-3, sub(10, 13));
    }
}