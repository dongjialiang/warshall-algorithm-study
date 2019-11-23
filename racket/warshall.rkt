#lang racket/base

(require racket/string)
(require data/gvector)

(define matrix (gvector)) ;;; 创建空的可变向量作为矩阵

(define (matrix-add matrix str-line)
    ;;; 添加一组向量作为矩阵一行数据的函数
    (cond [(not (string=? str-line "")) ;;; 字符串不为空才对字符串进行操作
        (gvector-add! matrix
            (list->gvector
                (for/list ([i (string-split str-line)])
                    (string->number i)
                )
            )
        )
    ])
)

(define (matrix-display matrix)
    ;;; 打印矩阵的函数
    (display "matrix:\n")
    (define N (gvector-count matrix))
    (do ((i 0 (+ i 1))) ((= i N))
        (do ((j 0 (+ j 1))) ((= j N))
            (display (format " ~a" (gvector-ref (gvector-ref matrix i) j)))
        )
        (display "\n")
    )
)

(define (warshall matrix)
    ;;; 执行warshall算法的函数
    (define N (gvector-count matrix))
    (for/gvector ([row matrix])
        (cond [(not (= N (gvector-count row)))
            (error "异常: 行数和列数不匹配!")
        ])
    )
    (do ((k 0 (+ k 1))) ((= k N))
        ;;; (matrix-display matrix) ;;; 取消该行注释可以观察关系矩阵的变化过程
        (do ((i 0 (+ i 1))) ((= i N))
            (cond [(= (gvector-ref (gvector-ref matrix i) k) 1) ;;; 当i到k有一条边时
                (do ((j 0 (+ j 1))) ((= j N))
                    (gvector-set! (gvector-ref matrix i) j ;;; 那么根据传递闭包的规则i到j也要补上一条边
                        (bitwise-ior
                            (gvector-ref (gvector-ref matrix i) j)
                            (gvector-ref (gvector-ref matrix k) j) ;;; 如果k到j也有一条边
                        )
                    )
                )
            ])
        )
    )
)

(define (read-line-loop str-line matrix)
    ;;; 循环读取行的函数
    (write '>)
    (set! str-line (read-line (current-input-port) 'any))

    (if (string-contains? str-line "ok")
        (begin
            (matrix-add matrix (string-replace str-line "ok" ""))
            (warshall matrix)
        )
        (begin
            (matrix-add matrix str-line)
            (read-line-loop str-line matrix)
        )
    )
)

(define (cli)
    ;;; 充当命令行界面入口的函数
    (define str-line "") ;;; 保存每行读取的字符串
    (display "请输入矩阵:\n")
    
    (read-line-loop str-line matrix)

    (matrix-display matrix)
)

(cli)
