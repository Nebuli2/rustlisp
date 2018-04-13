
(define (when pred body...)
    (if pred 
        (last body)
        empty))

(define (unless pred body...)
    (if (not pred)
        (last body)
        empty))

(define (check-sanity)
    (unless (pos? -5)
        (println "hi")
        (println "there")))

(check-sanity)