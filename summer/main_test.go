package main

import (
	"testing"
)

func TestSum(t *testing.T) {
	numbers := []float64{
		1.5,
		2.5,
		3.75,
		4.25,
	}
	if sum(numbers) != 12.0 {
		t.Error("Should be equal to 12.0")
	}
}
