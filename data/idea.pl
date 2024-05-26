pure func square(x: Int64) -> Int64 {
    return x * x
}

pure func add(x: Int64, y: Int64) -> Int64 {
    return x + y
}

func main() {
    let input: Int64[8] = [...]
    let squares = @map(square, input)
    let sum = @reduce(add, 0, squares)
}
