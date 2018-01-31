;; A point is a (make-point num num)
(define-struct point [x y])

;; point-mag : point -> num
(define (point-mag pt)
    (let ([x (point-x pt)]
          [y (point-y pt)])
        (sqrt (+ (* x x) (* y y)))))

;; point-dir : point -> num
(define (point-dir pt)
    (let ([x (point-x pt)]
          [y (point-y pt)])
        (atan2 y x)))

;; point-dist : point point -> num
(define (point-dist pt1 pt2)
    (let ([x1 (point-x pt1)]
          [y1 (point-y pt1)]
          [x2 (point-x pt2)]
          [y2 (point-y pt2)]
          [dx (- x1 x2)]
          [dy (- y1 y2)])
        (sqrt (+ (* dx dx) (* dy dy)))))

;; mean : num... -> num
(define (mean vals)
    (cond [(empty? vals) 0]
          [else (/ (sum vals) (len vals))]))

(define (point-disp pt)
    (format "(#{ (point-x pt) }, #{ (point-y pt) })"))

;; mid-point : point point -> point
(define (mid-point pt1 pt2)
    (let ([x1 (point-x pt1)]
          [y1 (point-y pt1)]
          [x2 (point-x pt2)]
          [y2 (point-y pt2)])
        (make-point
            (mean (list x1 x2))
            (mean (list y1 y2)))))
