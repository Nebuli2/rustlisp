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
    (foldr (lambda [val lst] (cons (func val) lst)) empty lst))

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

;; Greeting
(print "Welcome to " lisp-name " v" lisp-version ".")

;; make-point : num num -> point
;; point-x : point -> num
;; point-y : point -> num
;; A point represents a pair of numeric coordinates named "x" and "y".
(define-struct 
    (point x y))

;; point-add : point point -> point
;; Determines the sum of two points, where the sum is the point whose
;; coordinates are equal to the two points' corresponding coordinates added
;; together.
(define (point-add p1 p2)
    (make-point
        (+ (point-x p1) (point-x p2))
        (+ (point-y p1) (point-y p2))))

;; point-disp : point -> str
;; Produces a str representation of a point in the form "(x, y)".
(define (point-disp pt)
    (concat "(" (point-x pt) ", " (point-y pt) ")"))

(define (disp val)
    (cond [(point? val) (point-disp val)]
          [else (concat val)]))

