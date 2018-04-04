;; point = (make-point x y)
(define-struct point [x y])

;; event = (make-spawn-event symbol shape)
;;       | (make-delete-event symbol)
;;       | (make-move-event symbol point)
;;       | (make-jump-event symbol point)
(define-struct spawn-event [tag shape])

;; handle-spawn-event : spawn-event -> ()
(define (handle-spawn-event event)
    ())

(define-struct delete-event [tag])

;; handle-delete-event : delete-event -> ()
(define (handle-delete-event event)
    ())

(define-struct move-event [tag delta])
(define (handle-move-event event)
    ())

(define-struct jump-event [tag to])
(define (handle-jump-event event)
    ()

;; shape = (make-circle num)
;;       | (make-rect num num)
(define-struct circle [radius])
(define-struct rect [width height])

;; handle-event : event -> ()
(define (handle-event event)
    (cond 
        [(spawn-event? event) (handle-spawn-event event)]
        [(delete-event? event) (handle-delete-event event)]
        [(move-event? event) (handle-move-event event)]
        [(jump-event? event) (handle-jump-event event)]))
