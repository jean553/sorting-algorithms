# sorting-algorithms

[![Build Status](https://travis-ci.org/jean553/sorting-algorithms.svg?branch=master)](https://travis-ci.org/jean553/sorting-algorithms)

![Image 1](sorting-algorithms/res/screenshot.png)

Sorting algorithms implementations.

## Installation and usage

```
cargo run [algorithm]
```

`algorithm` can be equal to:
 * `insertion`: insertion sort
 * `selection`: selection sort
 * `bubble`: bubble sort

Use 'Space' key to iterate and sort step by step.

## Generate documentation

```
cargo rustdoc -- --no-defaults
```

## Sorting algorithms

This project contains the implementations of:
 * the insertion sort
 * the selection sort
 * the merge sort (separated project here [merge-sort](https://github.com/jean553/merge-sort))
 * the quick sort (separated project here [quick-sort](https://github.com/jean553/quick-sort))
 * the bubble sort

### Insertion sort

![Image 2](sorting-algorithms/res/insertion_sort.png)

More details: https://en.wikipedia.org/wiki/Insertion_sort

### Selection sort

One of the simpliest sort algorithms. The selection sort iterates on every item one by one.
For each item, it browses the whole array in order to find a smaller item than the current one.
When the whole array has been browsed, the current item is inverted with the found minimum value
(if one value has been found).

### Merge sort

A merge sort consists of a division of an array into smaller arrays.
This division is applied recursively until each sub-array contains exactly 1 or 2 items.
The items are ordered. Each sub-array is then merged with each other.
The merge sort is a `divide and conquer` algorithm.

### Bubble sort

The bubble sort is one of the worstest sorting algorithms.
It iterates on every item one by one and inverts it with the item just before if the two items are not ordered to each other.
The algorithm browses the whole array multiple times as long as all the items are ordered.
