package process

import (
	"encoding/csv"
	"errors"
	"fmt"
	"io"
	"log"
	"os"

	"github.com/lborres/aoc24/internal/conv"
)

func isSafe(report []int) (bool, error) {
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

// CheckLevels is the main process to solve day 02 part 01
func CheckLevels(inputFile string) {
  result := 0
  file, err := os.Open(inputFile)
  if err != nil {
    log.Println(err)
  }
  defer file.Close()

  csvReader := csv.NewReader(file)
  for {
    rawLine, err := csvReader.Read()
    if err == io.EOF {
      break
    }
    // WARN: csvReader.Read() expects all fields to have the same number of elements
    // below will disregard "malformed" rows and continue processing
    if err != nil && !errors.Is(err, csv.ErrFieldCount) {
      log.Println(err)
    }

    // convert raw line into []int
    line := conv.Arrtoi(rawLine)

    chk, err := isSafe(line)
    if err != nil {
      log.Println(err)
    }
    if chk {
      result++
    }
  }

  fmt.Println(result)
}
