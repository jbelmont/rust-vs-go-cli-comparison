package main

import (
	"fmt"
	"os"
	"strconv"
)

func sum(numbers []float64) float64 {
	sum := 0.0
	for _, num := range numbers {
		sum += num
	}
	return sum
}

func main() {
	// Here we parse the rest of the command line arguments since there is no flag.
	listOfNumbers := os.Args[1:]
	var numbers []float64
	for _, num := range listOfNumbers {
		number, err := strconv.ParseFloat(num, 64)
		if err != nil {
			fmt.Println(err)
			fmt.Printf("You did not provide a number to convert")
			os.Exit(1)
		}
		numbers = append(numbers, float64(number))
	}
	summation := sum(numbers)
	fmt.Println("Summation is: ", summation)
}
