
pub fn bubble_sort (array: &mut Vec <i32>) -> Vec <i32>
{
    let mut sorted;

    for i in (1..=array.len () - 1).rev () {
        sorted = true;
        for j in 0..i {
            if array[j] > array[j + 1] {
                sorted = false;
                array.swap (j, j + 1);
            }
        }
        if sorted {
            break;
        }
    }

    array.to_vec ()
}
