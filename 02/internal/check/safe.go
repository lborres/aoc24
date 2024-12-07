package check

func IsSafe(report []int) (bool, error) {
  var direction int
  for i := range report {
    if i == 0 {
      if report[i] < report[i+1] {
        direction = -1
      } else {
        direction = 1
      }
      continue
    }
    variance := report[i-1] - report[i]
    switch direction {
      case 1:
      if variance > 3 || variance <= 0{
        return false, nil
      }
      case -1:
      if variance < -3 || variance >= 0{
        return false, nil
      }
    }
  }

  return true, nil
}
