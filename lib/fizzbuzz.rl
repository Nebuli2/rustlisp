(define (do-line n)
    (println
        (cond [(eq? (modulo n 15) 0) "FizzBuzz"]
            [(eq? (modulo n 3) 0) "Fizz"]
            [(eq? (modulo n 5) 0) "Buzz"]
            [else n])))

(for-each do-line (range 1 101))