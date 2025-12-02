package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func loadFile(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer func(){
		err := file.Close()
		if err != nil {
			panic(err)
		}
	}()

	scanner := bufio.NewScanner(file)
	res := []string{}

	for scanner.Scan() {
		res = append(res, scanner.Text())
	}
	return res, nil	
}


func main() {
	data, err := loadFile("file.txt")
	if err != nil {
		panic(err)
	}
	startPos := 50

	zeros := 0

	pos := startPos
	for _, turn := range data{
		pos, err := handleTurn(turn, &pos)

		if err != nil {
			panic("Atoi threw an error")
		}
		if pos == 0 {
			zeros++
		}
	}

	fmt.Println("========RESULT=======")
	fmt.Println(zeros)
}

func handleTurn(turn string, pos *int) (int, error) {
	numTurns, err := strconv.Atoi(turn[1:])
	newPos := pos
	if err != nil {
		return 0, err
	}
	switch turn[0] {
	case 'L':
		*newPos -= numTurns
		for *newPos < 0 {
			*newPos = 100 + *newPos
		}
	case 'R':
		*newPos += numTurns
		for *newPos >= 100 {
			*newPos = *newPos - 100
		}
	}

	return *newPos, nil
}

