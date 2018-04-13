
(define (when pred body...)
    (if pred 
        (last body)
        empty))

(define (unless pred body...)
    (if (not pred)
        (last body)
        empty))