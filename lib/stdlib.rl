
(define (when pred body...)
    (if pred 
        (apply begin body)
        empty))

(define (unless pred body...)
    (if (not pred)
        (apply begin body)
        empty))