(println "Loading stdlib...")
(include "lib/stdlib.rl")

(println "Loading point struct...")
(include "lib/point.rl")

;; Greeting
(printf "Welcome to #{env/lisp-name} v#{env/lisp-version}.\n")