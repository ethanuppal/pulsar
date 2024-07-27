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

# current plan

we're going to make a dumb unified buffer that only supports constant for loop bounds

some well-formed/canonicalize AST pass? to ensure stuff is ok? ignoring structs for now we should be having for example the exact array accesses on an array, not gonna allow partial access currying type stuff yet (though it would be lazy compilation I think)

ok what's the ideal IR
the ideal ir is a series of assignments between cells....wait
this is kinda like calyx
hmmm but like
i ideally would like API enforcement of like only arrays can get accessed
which doesn't work for general cells
i digress ig?
but like i might stick to wanting API hand-holding too much
it's really useful for me though lol

another issue is i need a better distinguishing between ports and cells
like pulsar reg cell is kinda like sysv logic where i'm not distinguishing between register and wire
also technically all memories will get replaced with reads from registers/input wires
oh crap writing memory back is bad since you can't agen that for perfect prefetch

also as a note what I realized obviously is that the agen is also gonna have to compute some values when they depend on the loop bounds but that's just data independent stuff that probably takes a few cycles if it has a multiplier but whatever

currently the IR gen stuff is really messy
like the gen function is messy, component is messy, etc


ok so here's the plan wrt ports
the main issue is that we need to guarantee that a port can always represent a single lvalue so that we can treat the IR as a standard software assignment IR where order of operands in assignment matters - since otherwise you have to treat it as a graph
so there are two main things w need to do for this
- one is to somehow guarantee that when an expression is an lvalue, the gen_expr function doesn't add any additional assignments.
- the other is to have some sort of way for ports to represent arbitrary extion of something including chaining of . etc which means like we have to probably streict like structs can't contain arrays yeah this is a good idea 

a BIG todo is
don't have ports on components be cells!!!!!
