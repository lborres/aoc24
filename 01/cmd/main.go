package main

import (
	"encoding/csv"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"sync"

	"day01/calc"
	"day01/sorter"
)

func run() error {
  args := os.Args[1:]
  if len(args) < 2 {
    return fmt.Errorf("missing input arguments")
  }

  // read input flag
  inputFile := flag.String("i", "", "relative path to input file")
  flag.Parse()

  file, err := os.Open(*inputFile)
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

  // NOTE: Sort the unsorted lists
  // Spawn go routines so that sorting can be done concurrently
  var sortedA, sortedB []int
	var errA, errB error
  var wg sync.WaitGroup
  wg.Add(2)

  go func() {
    defer wg.Done()
    sortedA, errA = sorter.QuickSort(rawA)
  }()
  go func() {
    defer wg.Done()
    sortedB, errB = sorter.QuickSort(rawB)
  }()

  wg.Wait()

  if errA != nil {
    log.Fatalln(err)
  }
  if errB != nil {
    log.Fatalln(err)
  }

  // NOTE: Final output
  result, err := calc.TotalDistance(sortedA, sortedB)
  if err != nil {
    log.Fatalln(err)
  }
  fmt.Printf("%d\n",result)

  return nil
}

func main() {
  if err := run(); err != nil {
    log.Fatalf("Error: %v\n", err)
  }
}
