;; empty? : [A] -> bool
;; Determines whether or not the specified list is empty or not.
(define (empty? lst)
    (eq? (len lst) 0))

(define (last lst)
    (nth lst (- (len lst) 1)))

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
    (if (empty? lst)
        lst
        (cons 
            (func (car lst))
            (map func (cdr lst)))))

;; flatmap : (A -> [B]) [A] -> [B]
(define (flatmap func lst)
    (if (empty? lst)
        lst
        (append
            (func (car lst))
            (flatmap func (cdr lst)))))

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
    (foldr greater math/-infinity lst))

;; min : [num] -> num
;; Determines the minimum of the specified list of numbers.
(define (min lst)
    (foldr lesser math/infinity lst))

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

;; nil? : A -> bool
;; Determines whether or not the specified value is nil, the empty list.
(define (nil? val)
    (if (cons? val)
        (empty? val)
        #f))

;; list : A... -> [A]
;; Wraps all arguments passed to this function in a list.
(define (list vals...)
    vals)

;; printf : str -> nil
;; Formats the specified str according to the format function and prints the
;; result.
(define (printf fmt)
    (print (format fmt)))

;; inc : num -> num
;; Produces the value of x incremented by one.
(define (inc x) 
    (+ x 1))

;; dec : num -> num
;; Produces the value of x decremented by one.
(define (dec x) 
    (- x 1))

;; double : num -> num
;; Produces the value of x doubled.
(define (double x) 
    (* x 2))

;; half : num -> num
;; Produces the value of x halved.
(define (half x) 
    (/ x 2))

;; square : num -> num
;; Produces the value of the num squared.
(define (square x) 
    (* x x))

;; range : num num -> [num]
;; Produces a list of numbers equal to [from, to), with a step size of 1.
(define (range from to)
    (cond [(> to from) (cons from (range (inc from) to))]
          [else empty]))

;; range-to : num -> [num]
;; Produces a list of numbers equal to [0, to) with a step size of 1.
(define (range-to to)
    (range 0 to))

(define (parse-eval str)
    (eval (parse str)))

(define (lisp-name)
    env/lisp-name)

(define (lisp-version)
    env/lisp-version)

(define (run-repl)
    (printf "Welcome to #{ (lisp-name) } v#{ (lisp-version) }.\n")
    (repl))

(define (repl)
    (print "> ")
    (let ([res (parse-eval (read-line))])
        (if (not (nil? res))
            (println res)
            empty))
    (repl))

(define (factorial n)
    (if (> n -1)
        (apply * (map inc (range-to n)))
        -1))

(define (reload)
    (include "lib/main.rl"))

(define (std/reload)
    (println "Reloading stdlib...")
    (include "lib/stdlib.rl"))