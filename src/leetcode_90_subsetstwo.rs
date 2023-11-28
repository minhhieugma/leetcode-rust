pub fn main() {
    let mut nums = vec![2, 1,123];
    println!("{:?}", nums);

    nums = sort(nums);
    println!("{:?}", nums);

    let (results, _) = backtrack(&nums, vec![], vec![], 0);

    println!("{:?}", results);
}

fn sort(mut nums: Vec<i32>)-> Vec<i32>{
    for i in 0..nums.len() - 1{
        for j in i + 1..nums.len(){
                
            if nums[i] > nums[j]{
                let val = nums[i];
                nums[i] = nums[j];
                nums[j] = val;
            }
        }
    }

    return nums;
}

fn backtrack(
    nums: &Vec<i32>,
    mut results: Vec<Vec<i32>>,
    mut subset: Vec<i32>,
    start: usize,
) -> (Vec<Vec<i32>>, Vec<i32>) {
    for i in (start as usize)..nums.len() {
        let val = nums[i];

        if i > start && val == nums[i - 1] {
            continue;
        }

        subset.push(val);
        results.push(subset.clone());

        let cur = backtrack(nums, results.clone(), subset.clone(), i + 1);

        results = cur.0;

        subset.pop();
    }

    return (results, subset);
}
