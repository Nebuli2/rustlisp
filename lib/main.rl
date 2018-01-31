(define (run)
    (println "Loading stdlib...")
    (include "lib/stdlib.rl")

    (println "Loading point struct...")
    (include "lib/point.rl")

    (printf "Welcome to #{env/lisp-name} v#{env/lisp-version}.\n"))

(run)