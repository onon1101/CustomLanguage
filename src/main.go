package main

import (
	"fmt"
	"os"

	"github.com/onon1101/CustomLanguage/src/lexer"
)

func main() {
	bytes, _ := os.ReadFile("examples/00.lang")
	source := string(bytes)
	tokens := lexer.Tokenize(source)

	fmt.Printf("Code: %s\n", source)
	for _, token := range tokens {
		token.Debug()

	}
}
