package main
import (
	"fmt"
	"bufio"
	"os"
)

func main() {
	var line_number = 0
	for true {
		reader = buifo.NewReader(os.Stdin)
		fmt.Print(line_number, ": ")
		input, _ := reader.ReadString('\n')
		fmt.Print(input)
	}
}
