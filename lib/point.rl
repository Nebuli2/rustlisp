;; point : [x:num y:num]
;; A point stores an x and a y-coordinate.
(define-struct point [x y])

;; binary-point+ : point point -> point
;; Produces the sum of the two specified points.
(define (binary-point+ pt1 pt2)
    (let ([x1 (point-x pt1)]
          [y1 (point-y pt1)]
          [x2 (point-x pt2)]
          [y2 (point-y pt2)]
          [x (+ x1 x2)]
          [y (+ y1 y2)])
        (make-point x y)))

;; point+ : point... -> point
;; Produces the sum of all the specified points.
(define (point+ pts...)
    (foldr binary-point+ (make-point 0 0) pts))