package main

import (
	"encoding/csv"
	"errors"
	"flag"
	"fmt"
	"io"
	"log"
	"os"

	"day02/internal/check"
	"day02/internal/conv"
)

var inputFile string

func checkArgs() ( error ) {
  args := os.Args[1:]
  if len(args) < 2 {
    return errors.New("missing input arguments")
  }

  // INFO: Read input flag(s)
  flag.StringVar(&inputFile, "i", "", "relative path to input file")
  flag.Parse()

  return nil
}

func run() error {
  result := 0
  if err := checkArgs(); err != nil { return err }

  file, err := os.Open(inputFile)
  if err != nil { return err }
  defer file.Close()

  csvReader := csv.NewReader(file)
  for {
    rawLine, err := csvReader.Read()
    if err == io.EOF {
      break
    }
    if err != nil && !errors.Is(err, csv.ErrFieldCount) { return err }

    fmt.Println(rawLine)

    // convert raw line into []int
    line := conv.Arrtoi(rawLine)

    chk, err := check.IsSafe(line)
    if err != nil { return err }
    if chk {
      result++
    }
  }

  fmt.Println(result)
  return nil
}

func main() {
  if err := run(); err != nil {
    log.Fatalln(err)
  }
}
