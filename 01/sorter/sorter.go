package sorter

func QuickSort(input []int) ( []int, error ) {
  // Base case: arrays of size 0 or 1 are already sorted
  if len(input) <= 1 {
    return input, nil
  }

  // Set pivot point to middle of array
  pivotIndex := len(input)/2
  pivotValue := input[pivotIndex]

  left := []int{}
  right := []int{}

  for i, val := range input {
    if i == pivotIndex {
      continue
    }

    if val <= pivotValue {
      left = append(left, val)
    } else {
      right = append(right, val)
    }
  }

  sortedLeft, err := QuickSort(left)
  if err != nil {
    return nil, err
  }

  sortedRight, err := QuickSort(right)
  if err != nil {
    return nil, err
  }

 return append(append(sortedLeft, pivotValue),sortedRight...), nil
}
