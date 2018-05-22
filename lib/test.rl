;; for-each : (A -> nil) [A] -> nil
(define (for-each func lst)
  (cond [(empty? lst) empty]
        [else (begin
          (func (first lst))
          (for-each func (rest lst)))]))