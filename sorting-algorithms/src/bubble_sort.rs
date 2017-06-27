//! Provides the bubble sort feature

/// Triggers one step of the bubble sort
pub fn iterate_over_bubble_sort(
    array: &mut [u8],
    index: &mut usize,
    swapped: &mut bool,
) {
    if array[*index - 1] > array[*index] {
        array.swap(
            *index - 1,
            *index,
        );
        *swapped = true;
    }

    *index += 1;
}
