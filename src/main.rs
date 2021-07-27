fn main() {
    let nums = vec![-1, 4, 2, 1, 9, 10];
    println!("{}", first_missing_positive(nums));

}

fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
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
    let tmp_sub = (subscript.clone() - 1) as usize;
    if nums[tmp_sub] <= 0 {
        return;
    }

    if nums[tmp_sub] > nums.len() as i32 {
        return;
    }

    if nums[tmp_sub] - 1 == subscript {
        return;
    }

    let 

            if (nums[i] - 1) != i as i32 {
                if nums[i] < nums.len() as i32  {
                    let tmp2 = (nums[i].clone() - 1) as usize;
                    let tmp = nums[tmp2];
                    nums[tmp2] = nums[i];
                    nums[i] = tmp;
                }
            }
        }
    
}