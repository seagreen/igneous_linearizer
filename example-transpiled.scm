(define (displayln x)
    (display x)
    (newline))

(define (square x) (* x x))

(define (sum-of-squares x y)
	(+ (square x) (square y)))

(define (f a)
	(sum-of-squares (+ a 1) (* a 2)))

(displayln (square 21))

(displayln (square (+ 2 5)))

(displayln (square (square 3)))

(displayln (sum-of-squares 3 4))

(displayln (f 5))

(displayln 1000)
