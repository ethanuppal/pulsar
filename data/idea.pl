func square(x: Int64) -> Int64 {
    return x * x
}

func add(x: Int64, y: Int64) -> Int64 {
    return x + y
}

func main() {
    let input: Int64[8] = [...]
    let squares = @map(square, input)
    let output = @reduce(add, 0, input)
}
