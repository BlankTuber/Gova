package main

import "fmt"

func main() {
	var integer int = 32
	var integer8 int8 = 8
	var integer16 int16 = 16
	var integer32 int32 = 32
	var integer64 int64 = 64

	var unsignedInteger uint = 32
	var unsignedInteger8 uint8 = 8
	var unsignedInteger16 uint16 = 16
	var unsignedInteger32 uint32 = 32
	var unsignedInteger64 uint64 = 64

	var decimal32 float32 = 0.01
	var decimal64 float64 = 0.1

	var boolean bool = true

	var string string = "true"

	fmt.Printf("Integer: %d\n", integer)
	fmt.Printf("Integer 8: %d\n", integer8)
	fmt.Printf("Integer 16: %d\n", integer16)
	fmt.Printf("Integer 32: %d\n", integer32)
	fmt.Printf("Integer 64: %d\n", integer64)
	fmt.Printf("\n")
	fmt.Printf("Unsigned Integer: %d\n", unsignedInteger)
	fmt.Printf("Unsigned Integer 8: %d\n", unsignedInteger8)
	fmt.Printf("Unsigned Integer 16: %d\n", unsignedInteger16)
	fmt.Printf("Unsigned Integer 32: %d\n", unsignedInteger32)
	fmt.Printf("Unsigned Integer 64: %d\n", unsignedInteger64)
	fmt.Printf("\n")
	fmt.Printf("Decimal 32: %.2f\n", decimal32)
	fmt.Printf("Decimal 64: %.2f\n", decimal64)
	fmt.Printf("\n")
	fmt.Printf("Boolean: %t\n", boolean)
	fmt.Printf("String: %s\n", string)
	fmt.Printf("\n")
}
