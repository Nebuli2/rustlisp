;; list : A... -> [A]
;; Converts a variadic into a list.
(define (list vals...)
    vals)

;; empty? : [A] -> bool
;; Determines whether or not the specified list is empty or not.
(define (empty? lst)
    (eq? (len lst) 0))

;; last : [A] -> A?
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
(define (map f lst)
    (foldr (lambda [el to] (cons (f el) to)) empty lst))
    
; (define (map func lst)
;     (if (empty? lst)
;         lst
;         (cons 
;             (func (car lst))
;             (map func (cdr lst)))))

;; flatmap : (A -> [B]) [A] -> [B]
(define (flat-map func lst)
    (if (empty? lst)
        lst
        (prepend-list
            (func (car lst))
            (flat-map func (cdr lst)))))

(define (list-to n)
    (if (eq? n 0)
        (list n)
        (append n (list-to (- n 1)))))

;; filter : (A -> bool) [A] -> [A]
;; Produces a filtered view of the specified list, containing only the elements
;; meeting the specified predicate function.
(define (filter pred lst)
    (if (empty? lst)
        lst
        (let ([el (first lst)]
              [filtered (filter pred (rest lst))])
            (if (pred el)
                (cons el filtered)
                filtered))))

;; for-each : (A -> void) [A] -> void
;; Executes the specified function for each element of the specified list.
(define (for-each func lst)
    (map func lst)
    empty)

;; reverse : [A] -> [A]
;; Produces a reversed copy of the specified list.
(define (reverse lst)
    (foldr append empty lst))

;; append-list : [A] [A] -> [A]
(define (append-list lst to)
    (foldr append to (reverse lst)))

;; prepend-list : [A] [A] -> [A]
(define (prepend-list lst to)
    (foldr cons to lst))