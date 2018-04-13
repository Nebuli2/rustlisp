;; even? : num -> bool
;; Determines whether or not the specified number is even.
(define (even? n)
    (eq? 0 (modulo n 2)))

;; odd? : num -> bool
;; Determines whether or not the specified number is odd.
(define (odd? n)
    (not (even? n)))

;; pos? : num -> bool
;; Determines whether or not the specified number is positive.
(define (pos? n)
    (> n 0))

;; neg? : num -> bool
;; Determines whether or not the specified number is negative.
(define (neg? n)
    (< n 0))

;; inc : num -> num
;; Produces the specified number incremented by 1.
(define (inc n)
    (+ n 1))

;; dec : num -> num
;; Produces the specified number decremented by 1.
(define (dec n)
    (- n 1))

;; sum : [num] -> num
;; Produces the sum of the specified list of numbers.
(define (sum lst)
    (apply + lst))

;; range : num num -> [num]
;; Produces a list of numbers equal to [from, to), with a step size of 1.
(define (range from to)
    (cond [(> to from) (cons from (range (inc from) to))]
          [else empty]))