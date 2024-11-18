package main

import (
	"fmt"
	"os"

	"github.com/onon1101/CustomLanguage/src/lexer"
)

func main() {

	args := os.Args[1:]
	if len(args) == 0 {
		panic("No file provided")
	}

	code_file := os.Args[1]
	bytes, _ := os.ReadFile(code_file)
	source := string(bytes)
	tokens := lexer.Tokenize(source)

	fmt.Printf("Code: %s\n", source)
	for _, token := range tokens {
		token.Debug()

	}
}
