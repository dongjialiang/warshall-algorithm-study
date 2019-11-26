use std::io;
use std::io::Write;

fn warshall(matrix: &mut Vec<i32>) -> usize {
    let n = matrix.len();
    let mut cols = 1;

    loop {
        let cols_square = cols * cols;
        if cols_square > n {
            panic!("异常: 行数和列数不匹配");
        } else if cols_square == n {
            break;
        }
        cols += 1;
    }
    let mut k = 0;
    let mut _i = 0;
    let mut _j = 0;

    while k < cols {
        _i = 0;
        while _i < cols {
            _j = 0;
            if matrix[_i * cols + k] == 1 {
                while _j < cols {
                    matrix[_i * cols + _j] |= matrix[k * cols + _j];
                    _j += 1;
                }
            }
            _i += 1;
        }
        k += 1;
    }

    cols
}

fn matrix_add(_buffer: &mut String, matrix: &mut Vec<i32>) {
    if _buffer != "" {
        let _split = _buffer.split(" ");
        for s in _split {
            let my_int: i32 = s.parse().expect("Not a number!");
            matrix.push(my_int);
        }
    }
}

fn read_input_loop() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut _buffer = String::new();
    let mut matrix = Vec::new();

    loop {
        _buffer = "".to_string();
        print!("> ");
        let _ = stdout.flush();
        stdin.read_line(&mut _buffer).expect("Error: read failed!");
        _buffer = _buffer.trim().to_string();
        if _buffer == ".exit" {
            println!(" 再见朋友, 欢迎下次使用");
            break;
        } else if _buffer.contains("ok") {
            _buffer = _buffer.replace("ok", "");
            matrix_add(&mut _buffer, &mut matrix);
            let length = warshall(&mut matrix);
            let mut _i = 0;
            let mut _j = 0;
            println!("matrix:");
            while _i < length {
                _j = 0;
                while _j < length {
                    print!("{} ", matrix[_i * length + _j]);
                    _j += 1;
                }
                print!("\n");
                _i += 1;
            }
            matrix = vec![];
            println!("\n请输入矩阵: (结尾写上ok表示结束)");
        } else {
            matrix_add(&mut _buffer, &mut matrix);
        }
    }
}

fn main() {
    println!("*要求输入矩阵中每行的元素以空格区分,并且元素以0, 1表示");
    println!("*输入`.exit`结束");
    println!("-----------------------------------");
    println!("\n请输入矩阵: (结尾写上ok表示结束)");

    read_input_loop();
}
