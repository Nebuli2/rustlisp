;; result<T, E> = ok<T>
;;              | err<E>

;; An ok<T> is a (make-ok val).
(define-struct ok [val])

;; An err<E> is a (make-err why).
(define-struct err [why])

;; map-ok : (T -> U) result<T, E> -> result<U, E>
(define (map-ok func res)
    (cond 
        [(is-ok? res) (func (ok-val res))]
        [else res]))

;; map-err : (E -> U) result<T, E> -> result<T, U>
(define (map-err func res)
    (cond  
        [(is-err? res) (func (err-why res))]
        [else res]))