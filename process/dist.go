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

// similarityScore() takes in 2 sorted arrays
// prints out similarity score as described by the problem
func similarityScore(arrA []int, arrB []int) error {
  result := 0
  ptrA := 0
  ptrB := 0
  sim := 0
  fmt.Printf("Checking Similarity for the following:\nLeft: %v\nRight: %v\n", arrA, arrB)

  // sample inputs
  // Left: [1 2 3 3 3 4]
  // Right: [3 3 3 4 5 9]
  for ptrA <= len(arrA) - 1 {
    fmt.Printf("%vx%v: %v\n", arrA[ptrA], arrB[ptrB], result)

    // INFO: If current value is same as previous, use the same similarity val
    if ptrA > 0 && arrA[ptrA] == arrA[ptrA - 1] {
      result = result + arrA[ptrA] * sim
      ptrA++
      continue
    }

    sim = 0
    for arrA[ptrA] == arrB[ptrB] && ptrB <= len(arrB) - 1 {
      sim++
      ptrB++
    }
    result += arrA[ptrA] * sim

    // INFO: Traverse arrB until we can start comparing
    if arrA[ptrA] > arrB[ptrB] {
      ptrB++
      continue
    }
    ptrA++
  }


  fmt.Printf("Similarity Score: %d\n",result)

  return nil
}

func totalDistance(arrA []int, arrB []int) error {
  result := 0

  for i := 0; i <= len(arrA) - 1; i++ {
    result += calc.AbsInt(arrA[i] - arrB[i])
  }

  fmt.Printf("Total Distance: %d\n",result)

  return nil
}

// CheckDist is the main process for the solution to day 01
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

  // INFO: Final output
  // Solution to Part 1
  if err := totalDistance(sortedA, sortedB); err != nil {
    log.Fatalln(err)
  }
  if err := similarityScore(sortedA, sortedB); err != nil {
    log.Fatalln(err)
  }

}
