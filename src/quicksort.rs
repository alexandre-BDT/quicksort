pub fn solve(numbers: &mut Vec<i32>) {
    let high = numbers.len() - 1;
    quicksort(numbers, 0, high);
}

fn quicksort(numbers: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let part = partition(numbers, low, high);
        quicksort(numbers, low, part - 1);
        quicksort(numbers, part + 1, high);
    }
}

fn partition(number: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = number[high];
    let mut i: usize = low;

    for x in low..high {
        if number[x] <= pivot {
            number.swap(i, x);
            i += 1;
        }
    }
    number.swap(i, high);
    return i;
}
