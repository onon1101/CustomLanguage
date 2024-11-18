package ast

import "github.com/onon1101/CustomLanguage/src/lexer"

type NumberExpr struct {
	Value float64
}

func (n NumberExpr) expr() {}

type StringExpr struct {
	Value string
}

func (s StringExpr) expr() {
}

type SymbolExpr struct {}

func (s SymbolExpr) expr() {}

type BinaryExpr struct {
	Left Expr
	Operator lexer.Token
	Right Expr
}

func (s BinaryExpr) expr() {}