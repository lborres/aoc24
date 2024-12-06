package main

import (
	"os"
	"testing"
)

func TestMain(t *testing.T) {
  originalArgs := os.Args // Save original os.Args
  defer func() { os.Args = originalArgs }() // restore original args after test
  // originalStdout := os.Stdout


}
