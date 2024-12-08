package main

import (
	"errors"
	"flag"
	"fmt"
	"log"
	"os"

	"github.com/lborres/aoc24/process"
)

var inputFile string

func run() error {
  // INFO: Read global flag specifying input file
  flag.StringVar(&inputFile, "i", "", "relative path to input file")
  flag.Parse()

  if len(os.Args) < 2 {
    return errors.New("Usage: mycli -i <file> <command> [options]")
  }

  if inputFile == "" {
    fmt.Printf("Error: -i or --input flag is required")
    return errors.New("Usage: mycli -i <file> <command> [options]")
  }

  // INFO: Define subcommands
  // TODO: Commented out unless subcommand has own set of flags
  // distCmd := flag.NewFlagSet("dist", flag.ExitOnError)
  // safeCmd := flag.NewFlagSet("safe", flag.ExitOnError)

  // INFO: Run subcommand process
  subcommand := os.Args[3]
  switch subcommand {
  case "dist":
    process.CheckDist(inputFile)
  case "safe":
    process.CheckLevels(inputFile)
  default:
    err := fmt.Sprintf("Unknown subcommand: %s\n", subcommand)
    return errors.New(err)
  }

  return nil
}

func main() {
  if err := run(); err != nil {
    log.Fatalln(err)
  }
}
