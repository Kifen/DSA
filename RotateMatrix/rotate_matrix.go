package RotateMatrix

import "fmt"

func RotateMatrix(matrix [][]int) {
	println(fmt.Sprintf("Input matrix: %v", matrix))
	// reverse matrix
	endArr := matrix[len(matrix) - 1] 
	matrix[len(matrix)-1] = matrix[0]
	matrix[0] = endArr

	for i := 0; i < len(matrix); i++ {
		for j := 0; j < i; j++ {
			temp := matrix[j][i]
			matrix[j][i] = matrix[i][j]
			matrix[i][j] = temp
		}
	}
	println(fmt.Sprintf("Output matrix: %v", matrix))
}