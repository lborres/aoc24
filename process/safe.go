package process

import (
  "encoding/csv"
  "errors"
  "fmt"
  "io"
  "log"
  "os"

  "github.com/ttacon/chalk"
  "github.com/lborres/aoc24/internal/conv"
)

func isSafe(report []int, useDamper bool) bool {
  fmt.Printf("Checking(dampened? %v): %v\n", !useDamper, report)
  var direction int
  var k int = 0
  newReport := []int{}
  for i := range report {
    if i == 0 {
      if report[i] < report[i+1] {
        direction = 1
      } else {
        direction = -1
      }
      continue
    }
    variance := report[i] - report[i-1]
    switch direction {
      case 1:
      if variance > 3 || variance <= 0{
        fmt.Printf("%sFAIL%s (+)(%v->%v: %v)\n", chalk.Red, chalk.Reset, report[i-1], report[i], variance)
        if useDamper {
          newReport = append(newReport, report[:i]...)
          if i < len(report) - 1 {
            newReport = append(newReport, report[i+1:]...)
          }
          return isSafe(newReport, false)
        }
        return false
      }
      case -1:
      if variance < -3 || variance >= 0{
        fmt.Printf("%sFAIL%s (-)(%v->%v: %v)\n", chalk.Red, chalk.Reset, report[i-1], report[i], variance)
        if useDamper {
          newReport = append(newReport, report[:i]...)
          if i < len(report) - 1 {
            newReport = append(newReport, report[i+1:]...)
          }
          return isSafe(newReport, false)
        }
        return false
      }
    }
    k++
  }

  fmt.Printf("%sPASS%s\n", chalk.Green, chalk.Reset)

  return true
}

// CheckLevels is the main process to solve day 02 part 01
func CheckLevels(inputFile string) {
  resultA := 0
  resultB := 0
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

    chkA := isSafe(line, false)
    if chkA {
      resultA++
    }
    chkB := isSafe(line, true)
    if chkB {
      resultB++
    }
  }

  fmt.Printf("Safe Reports(undampend): %v\n", resultA)
  fmt.Printf("Safe Reports(dampened): %v\n", resultB)
}
