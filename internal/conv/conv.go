package conv

import (
	"log"
	"strconv"
)

func Arrtoi(input []string) []int {
  var result []int

  for _, val := range input {
    newVal, err := strconv.Atoi(val)
    if err != nil {
      log.Fatalln(err)
    }

    result = append(result, newVal)
  }
  return result
}
