package calc

func abs(val int) int {
  if val < 0 {
    return -val
  }
  return val
}

func TotalDistance(arrA []int, arrB []int) ( int, error ){
  result := 0

  for i := 0; i <= len(arrA) - 1; i++ {
    result += abs(arrA[i] - arrB[i])
  }

  return result, nil
}
