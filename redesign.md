```swift
func gmm(a: Int[128][128], b: Int[128][128]) -> (c: Int[128][128]) {
    for i in 0 ..< 128 {
        for j in 0 ..< 128 {
            for k in 0 ..< 128 {
                c[i][k] += a[i][j] * b[j][k]
            }
        }
    }
}

struct Complex {
    re: Double,
    im: Double
}

func fft<FFT_SIZE>(input: Complex[FFT_SIZE], twid: Complex[FFT_SIZE / 2]) -> (output: Int) {
    for log in 0 ..< LOG2_FFT_SIZE {
        let span = FFT_SIZE >> (log + 1)
        for odd_ in span ..< FFT_SIZE {
            let odd = odd_ | span
            let even = odd ^ span
            let temp = input[even].re + input[odd].re
            ---
            input[odd].re = input[even].re - input[odd].re
            ---
            input[even].re = temp
            // ...
        }
    }
}
```
