#!/usr/local/bin/rlisp

(define (gwf die)
    (let ([avg (+ (/ die 2) 0.5)]
          [others (skip 2 (range 1 (+ die 1)))])
        (/ (foldr + (* avg 2) others) die)))

(define d4 (gwf 4))
(define d6 (gwf 6))
(define d8 (gwf 8))
(define d10 (gwf 10))
(define d12 (gwf 12))

(println `avg(d4) = ${d4}`)
