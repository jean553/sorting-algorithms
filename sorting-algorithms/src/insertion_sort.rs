/// Triggers one step of the insertion sort
pub fn iterate_over_insertion_sort(
    array: &mut [u8; 50], 
    a: &mut usize,
    b: &mut usize,
)
{
    *b = *a;

    while *b > 0 && array[*b - 1] > array[*b] {
        array.swap(*b, *b - 1);
        *b -= 1;
    }

    *a += 1;
}
