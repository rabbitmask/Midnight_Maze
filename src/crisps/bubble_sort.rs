use rand::Rng;

fn generate_random_numbers(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..100)).collect()
}

fn bubble(numbers: &mut [i32]) {
    let n = numbers.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort_demo(){
    let mut numbers = generate_random_numbers(10);
    println!("生成的随机数：{:?}", numbers);

    bubble(&mut numbers);
    println!("排序后的结果：{:?}", numbers);
}

pub(crate) fn bubble_sort(n: usize){
    let mut numbers = generate_random_numbers(n);
    // println!("生成的随机数：{:?}", numbers);

    bubble(&mut numbers);
    // println!("排序后的结果：{:?}", numbers);
}
