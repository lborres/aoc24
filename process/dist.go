package process

import (
	"encoding/csv"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"sync"

	"github.com/lborres/aoc24/internal/alg"
	"github.com/lborres/aoc24/internal/calc"
)

func totalDistance(arrA []int, arrB []int) ( int, error ){
  result := 0

  for i := 0; i <= len(arrA) - 1; i++ {
    result += calc.AbsInt(arrA[i] - arrB[i])
  }

  return result, nil
}

// DistMain is the main process for the solution to day 01 part 01
func CheckDist(inputFile string) {
  file, err := os.Open(inputFile)
  if err != nil {
    log.Fatalln(err)
  }
  defer file.Close()

  var rawA, rawB []int

  csvReader := csv.NewReader(file)
  for {
    rec, err := csvReader.Read()
    if err == io.EOF {
      break
    }
    if err != nil {
      log.Fatalln(err)
    }

    colA, err := strconv.Atoi(rec[0])
    if err != nil {
      log.Fatalln(err)
    }

    colB, err := strconv.Atoi(rec[1])
    if err != nil {
      log.Fatalln(err)
    }

    rawA = append(rawA, colA)
    rawB = append(rawB, colB)
  }

  // INFO: Sort the unsorted lists
  // Spawn go routines so that sorting can be done concurrently
  var sortedA, sortedB []int
	var errA, errB error
  var wg sync.WaitGroup
  wg.Add(2)

  go func() {
    defer wg.Done()
    sortedA, errA = alg.QuickSort(rawA)
  }()
  go func() {
    defer wg.Done()
    sortedB, errB = alg.QuickSort(rawB)
  }()

  wg.Wait()

  if errA != nil {
    log.Fatalln(err)
  }
  if errB != nil {
    log.Fatalln(err)
  }

  // NOTE: Final output
  result, err := totalDistance(sortedA, sortedB)
  if err != nil {
    log.Fatalln(err)
  }
  fmt.Printf("%d\n",result)
}
