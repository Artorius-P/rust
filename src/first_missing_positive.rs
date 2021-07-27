fn main() {
    let nums = vec![1];
    println!("{}", first_missing_positive(nums));

}

fn first_missing_positive(mut nums: Vec<i32>) -> i32 {

    let times_flag = 0;
    for i in 0..nums.len() {
        let element = nums[i].clone();
        swap(&mut nums, i as i32, element, times_flag);
    }

    for i in 0..nums.len() {
        if nums[i] - 1 != i as i32 {
            return (i + 1)as i32;
        }
    }
    return (nums.len()+1) as i32;
}


fn swap(nums: &mut Vec<i32>, subscript: i32, element: i32, mut times_flag: i32) {

    times_flag += 1;

    if times_flag > nums.len() as i32 {
        return;
    }
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
    let tmp = nums[(element-1) as usize];
    nums[(element-1) as usize] = element;
    nums[subscript as usize] = tmp;
    
    swap(nums, subscript, nums[subscript as usize], times_flag);
}