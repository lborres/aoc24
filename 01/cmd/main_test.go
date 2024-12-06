package main

import (
	"bytes"
	"fmt"
	"log"
	"os"
	"testing"
)

func TestRun(t *testing.T) {
  originalArgs := os.Args // Save original os.Args
  defer func() { os.Args = originalArgs }() // restore original args after test
  originalStdout := os.Stdout

  r, w, err := os.Pipe()
  if err != nil {
    log.Fatalln("Unexpected Error", err)
  }
  os.Stdout = w

  // arrange 
  os.Args = []string{"program","-i","../input/sample.csv"}
  main()

  w.Close()

  var buf bytes.Buffer
  buf.ReadFrom(r)
  capturedOutput := buf.String()
  fmt.Println(capturedOutput)

  // Restore original os.Stdout
  os.Stdout = originalStdout

  // assert
  expected := "11\n"
  if capturedOutput != expected {
    t.Errorf("expected %q, got %q", expected, capturedOutput)
  }
}
