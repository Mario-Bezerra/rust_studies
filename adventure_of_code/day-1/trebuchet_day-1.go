package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	totalSum := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		firstNum, lastNum := findFirstAndLastNumber(line)

		concatNum, err := strconv.Atoi(firstNum + lastNum)
		if err == nil {
			totalSum += concatNum
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	fmt.Println("Total sum of concatenated first and last numbers in the file:", totalSum)
}

func findFirstAndLastNumber(s string) (string, string) {
	var firstNum, lastNum string
	foundFirst := false

	for _, char := range s {
		if unicode.IsDigit(char) {
			if !foundFirst {
				firstNum = string(char)
				foundFirst = true
			}
			lastNum = string(char)
		}
	}
	return firstNum, lastNum
}
