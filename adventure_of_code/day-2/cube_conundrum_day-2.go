package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	var possibles int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		var game = strings.Split(line, ":")
		id, err := strconv.Atoi(game[0][5:])
		if err != nil {
			fmt.Println("Erro ao converter:", err)
			return
		}
		var contain_true int
		var combinations = strings.Split(game[1], ";")

		for _, combination := range combinations {
			var contain = strings.Split(combination, ",")
			valid := true

			for _, set := range contain {
				var cube = strings.Split(set, " ")
				var number = cube[1]
				var collor = cube[2]
				number_parsed, err := strconv.Atoi(number)
				if err != nil {
					fmt.Println("Erro ao converter:", err)
					return
				}

				if !checkValidation(number_parsed, collor) {
					valid = false
					break
				}
			}

			if valid {
				contain_true++
			}
		}

		if contain_true == len(combinations) {
			possibles += id
		}
	}

	fmt.Println(possibles)
}

func checkValidation(number int, collor string) bool {
	switch collor {
	case "red":
		if number > 12 {
			return false
		}
	case "blue":
		if number > 14 {
			return false
		}
	case "green":
		if number > 13 {
			return false
		}
	default:
		return false
	}
	return true
}
