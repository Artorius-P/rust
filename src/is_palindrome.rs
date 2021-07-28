fn main() {
    let num = -123121;
    println!("{}", is_palindrome(num));
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut tmp = x.clone();
    let mut reverse = 0;

    while tmp != 0 {
        let bit = tmp % 10;
        tmp /= 10;
        reverse = reverse * 10 + bit;
    }

    if reverse == x {
        return true;
    }

    return false;
}