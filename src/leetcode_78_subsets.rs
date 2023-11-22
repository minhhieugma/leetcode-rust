
pub fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", nums);

    let (results, _) = backtrack(&nums, vec![vec![]], Vec::new(), 0);

    println!("{:?}", results);
}

fn backtrack(
    nums: &Vec<i32>,
    results: Vec<Vec<i32>>,
    subset: Vec<i32>,
    start: i32,
) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut r_results: Vec<Vec<i32>> = results.clone();
    let mut r_subset: Vec<i32> = subset.clone();

    for i in (start as usize)..(nums.len()) {
        let cur = nums[i];
        println!(
            "------------------start:{} i:{} cur {}------------------",
            start, i, cur
        );

        r_subset.push(cur);
        r_results.push(r_subset.clone());
        println!("r_results->{:?}", r_results);

        let (a, b) = backtrack(&nums, r_results.clone(), r_subset.clone(), i as i32 + 1);
        r_results = a;
        r_subset = b;

        r_subset.pop();
    }

    return (r_results, r_subset);
}
