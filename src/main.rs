fn main() {
    let nums = vec![-1, 4, 2, 1, 9, 10];
    println!("{}", first_missing_positive(nums));

}

fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        let element = nums[i].clone();
        swap(&mut nums, i as i32, element);
    }
    for i in 0..nums.len() {
        print!("{} ", nums[i]);
    }
    for i in 0..nums.len() {
        if nums[i] - 1 != i as i32 {
            return (i + 1)as i32;
        }
    }
    return nums.len()as i32;
}

fn swap(nums: &mut Vec<i32>, subscript: i32, element: i32) {

    //元素不在范围内
    if element <= 0 {
        return;
    }

    if element > nums.len() as i32 {
        return;
    }

    //下标里的数-1正好与下标相等
    if element - 1 == subscript {
        return;
    }

    //元素-1与下标不等，
    let tmp = nums[element as usize];
    nums[element as usize] = element;
    nums[subscript as usize] = tmp;
    
    swap(nums, subscript, nums[subscript as usize]);
}