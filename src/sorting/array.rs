pub fn swap(mut arr: Vec<i32>, index: usize, index2: usize) ->  Vec<i32> {
    if index == index2 {
        return arr;
    }
    let tmp = arr[index];
    arr[index] = arr[index2];
    arr[index2] = tmp;
    arr
}