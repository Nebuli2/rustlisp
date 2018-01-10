;; empty? : [A] -> bool
;; Determines whether or not the specified list is empty or not.
(define (empty? l)
    (eq? (len l) 0))

;; foldr : (A B -> A) A [B] -> A
;; Performs a right-associative fold on the specified list, with the specified
;; accumulator and function.
(define (foldr func acc lst)
    (if (empty? lst)
        acc
        (func (car lst) (foldr func acc (cdr lst)))))

;; foldl : (A B -> A) A [B] -> A
;; Performed a left-associative fold on the specified list, with the specified
;; accumulator and function.
(define (foldl func acc lst)
    (if (empty? lst)
        acc
        (foldl func (func acc (car lst) (cdr lst)))))

;; map : (A -> B) [A] -> [B]
;; Maps the specified function to the specified list, producing a list
;; containing the mapped results.
(define (map func lst)
    (foldr (lambda [val lst] (cons (func val) lst) [] lst)))

;; ormap : (A -> bool) [A] -> bool
;; Determines whether or not at least one element of the list satisfies the
;; specified condition. An empty list will always return false.
(define (ormap func lst)
    (cond [(empty? lst) #f]
          [(func (car lst)) #t]
          [else (ormap func (cdr lst))]))

;; andmap : (A -> bool) [A] -> bool
;; Determines whether or not all of the elements of the list satisfy the
;; specified condition. An empty list will always return true.
(define (andmap func lst)
    (cond [(empty? lst) #t]
          [(not (func (car lst))) #f]
          [else (andmap func (cdr lst))]))

;; greater : num num -> num
;; Determines the greater of two numbers.
(define (greater a b)
    (if (> b a)
        b
        a))

;; lesser : num num -> num
;; Determines the lesser of two numbers.
(define (lesser a b)
    (if (> a b)
        b
        a))

;; max : [num] -> num
;; Determines the maximum of the specified list of numbers.
(define (max lst)
    (foldr greater -infinity lst))

;; min : [num] -> num
;; Determines the minimum of the specified list of numbers.
(define (min lst)
    (foldr lesser infinity lst))

;; sum : [num] -> num
;; Determines the sum of the specified list of numbers.
(define (sum lst)
    (foldr + 0 lst))

;; first : [A] -> A
;; Produces the first element of the specified list.
(define (first lst)
    (car lst))

;; rest : [A] -> [A]
;; Produces the rest of the list after the first element.
(define (rest lst)
    (cdr lst))

;; for-each : (A -> nil) [A] -> nil
;; Performs the specified function on all members of the specified list.
(define (for-each func lst)
    (if (empty? lst)
        empty
        (begin
            (func (car lst))
            (for-each func (cdr lst)))))

;; fib : num -> num
(define (fib n)
    (cond [(< n 0) -1]
          [(< n 2) n]
          [else (+ 
            (fib (- n 1)) 
            (fib (- n 2)))]))