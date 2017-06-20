//! Provides the selection sort feature

/// Triggers one step of the selection sort
pub fn iterate_over_selection_sort(
    array: &mut [u8; 50], 
    index: &mut usize,
)
{
    let mut min = *index;
    let start = *index + 1;

    for i in start..50 {

        if array[i] < array[min] {
            min = i;
        }
    }

    if min != *index {
        array.swap(
            *index,
            min,
        );
    }

    *index += 1;
}
