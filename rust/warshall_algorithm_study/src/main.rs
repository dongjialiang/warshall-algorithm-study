use std::io;
use std::io::Write;

fn warshall(matrix: &mut Vec<i32>) -> usize {
    let length = matrix.len();
    let mut n = 1;

    loop {
        let n_square = n * n;
        if n_square > length {
            println!("异常: 行数和列数不匹配");
            println!("\n请输入矩阵: (结尾写上ok表示结束)");
            read_input_loop();
        } else if n_square == length {
            break;
        }
        n += 1;
    }
    let mut _k = 0;
    let mut _i = 0;
    let mut _j = 0;

    while _k < n {
        _i = 0;
        while _i < n {
            _j = 0;
            if matrix[_i * n + _k] == 1 {
                while _j < n {
                    matrix[_i * n + _j] |= matrix[_k * n + _j];
                    _j += 1;
                }
            }
            _i += 1;
        }
        _k += 1;
    }

    n
}

fn matrix_add(_buffer: &mut String, matrix: &mut Vec<i32>) {
    if _buffer != "" {
        let _split = _buffer.split(" ");
        for ch in _split {
            let weight: i32 = ch.parse().expect("Not a number!");
            matrix.push(weight);
        }
    }
}

fn matrix_show(n: usize, matrix: &Vec<i32>) {
    let mut _i = 0;
    let mut _j = 0;
    println!("matrix:");
    while _i < n {
        _j = 0;
        while _j < n {
            print!("{} ", matrix[_i * n + _j]);
            _j += 1;
        }
        print!("\n");
        _i += 1;
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
        let _flush = stdout.flush();
        stdin.read_line(&mut _buffer).expect("Error: read failed!");
        _buffer = _buffer.trim().to_string();
        if _buffer == ".exit" {
            println!(" 再见朋友, 欢迎下次使用");
            std::process::exit(1);
        } else if _buffer.contains("ok") {
            _buffer = _buffer.replace("ok", "");
            matrix_add(&mut _buffer, &mut matrix);
            
            matrix_show(warshall(&mut matrix), &matrix);
            
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
