package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	runes := readFile()

	var validNumber []string

	for i := 0; i < len(runes); i++ {
		var isValidDigit bool = false
		var number string = ""

		for j := 0; j < len(runes[i]); j++ {
			if unicode.IsDigit(runes[i][j]) {
				if isValidDigit {
					number += string(runes[i][j])
				} else {
					number = string(runes[i][j])
					if checkAdjacentSymbols(runes, i, j) {
						isValidDigit = true
					}
				}
			} else {
				if number != "" {
					validNumber = append(validNumber, number)
					number = ""
					isValidDigit = false
				}
			}
		}

		if number != "" {
			validNumber = append(validNumber, number)
		}
	}

	result := sumValues(validNumber)
	fmt.Println(result)
}

func sumValues(validNumbers []string) int {
	sum := 0
	for _, str := range validNumbers {
		num, err := strconv.Atoi(str)
		if err != nil {
			fmt.Printf("Error converting string to integer: %s\n", str)
			continue
		}
		sum += num
	}
	return sum
}

func readFile() [][]rune {
	file := openFile()

	charArray := createCharArray()

	linePosition := 0
	scanner := bufio.NewScanner(file)
	extractRunes(scanner, charArray, linePosition)

	return charArray
}

func extractRunes(scanner *bufio.Scanner, charArray [][]rune, linePosition int) {
	for scanner.Scan() {
		line := scanner.Text()
		r := bufio.NewReader(strings.NewReader(line))
		for {
			if char, _, err := r.ReadRune(); err != nil {
				if err == io.EOF {
					break
				} else {
					log.Fatal(err)
				}
			} else {
				charArray[linePosition] = append(charArray[linePosition], char)
			}
		}
		linePosition++
	}
}

func openFile() *os.File {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal("Error opening file:", err)
	}
	defer file.Close()
	return file
}

func createCharArray() [][]rune {
	charArray := make([][]rune, 0)
	return charArray
}

func isValid(x, y, rows, cols int) bool {
	return x >= 0 && y >= 0 && x < rows && y < cols
}

func checkAdjacentSymbols(matrix [][]rune, x, y int) bool {
	rows := len(matrix)
	if rows == 0 {
		return false
	}
	cols := len(matrix[0])

	isSymbol := regexp.MustCompile(`[^a-zA-Z0-9\s.]`)

	dx := []int{-1, 1, 0, 0}
	dy := []int{0, 0, -1, 1}

	for i := 0; i < 4; i++ {
		newX := x + dx[i]
		newY := y + dy[i]

		if isValid(newX, newY, rows, cols) && isSymbol.MatchString(string(matrix[newX][newY])) {
			return true
		}
	}
	return false
}
