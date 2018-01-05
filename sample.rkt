(define (empty? l)
    (eq? (len l) 0))

;; foldr : (A B => A) A list[B] => A
(define (foldr func acc lst)
    (if (empty? lst)
        acc
        (func (car lst) (foldr func acc (cdr lst)))))

(define (map func lst)
    (foldr (lambda [val lst] (cons (func val) lst) [] lst)))

(define (max a b)
    (if (> a b)
        a
        b))

(define (square x)
    (* x x))

(define (first-num a b c d)
    (cond [(num? a) 0]
          [(num? b) 1]
          [(num? c) 2]
          [(num? d) 3]
          [else (begin
            (print "No nums found"))]))

(define (foo)
    (let ([x 5]
          [y 12])
        (begin
            (print "X = " x)
            (print "Y = " y)
            (+ x y))))