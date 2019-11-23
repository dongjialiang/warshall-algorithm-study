#!/usr/bin/env node

const warshall = require('./warshall');

(function warshallREPL() {
    'use strict';
    const stdout = process.stdout;
    const resetMatrix = (matrix) => { // 重置矩阵的函数
        const colLength = matrix[0].length;
        for (let index = 0; index < colLength + 1; index++) {
            stdout.write(`________`);
        }
        stdout.write(`\n`);
        while (matrix.length > 0) {
            matrix.pop();
        }
        rowIndex = 0;
        stdout.write('请输入矩阵: (结尾写上ok表示结束)\n> ');
    }
    const interfaceShow = (TRMatrix) => { // 显示界面(矩阵表格样式)的函数
        const colLength = TRMatrix[0].length;
        for (let index = 0; index < colLength + 1; index++) {
            stdout.write(`________`);
        }
        stdout.write('\n  \\ col|');
        for (let index = 0; index < colLength; index++) {
            stdout.write(`${index + 1}      |`);
        }
        stdout.write(`\nrow\\   |`);
        for (let index = 0; index < colLength; index++) {
            stdout.write(`_______|`);
        }
        stdout.write(`\n`);

        TRMatrix.map((rowArray, row) => {
            stdout.write(`${row + 1}      |`);
            rowArray.map((value) => {
                stdout.write(`${value}      |`);
            });
            stdout.write(`\n`);
        });
    }
    const stdin = process.stdin;

    stdin.setEncoding('utf-8');

    const matrix = []; // 矩阵
    let rowIndex = 0;  // 行索引
    stdout.write('*要求输入矩阵中每行的元素以空格区分,并且元素以0, 1表示\n');
    stdout.write('*输入`.exit`结束\n');
    stdout.write('-----------------------------------\n');
    stdout.write('请输入矩阵: (结尾写上ok表示结束)\n> ');
    stdin.on('data', line => {
        const lineString = line.toString().replace(/[\r\n]+/, ''); // 获取每行的实际字符串

        if (lineString === '.exit') {
            stdout.write('  再见朋友, 欢迎你下次使用~');
            process.exit();
        } else if (line.includes('ok')) {
            const lastLine = lineString.replace('ok', '');
            if (lastLine.length !== 0) {
                matrix[rowIndex] = lineString.replace('ok', '').split(' '); // 矩阵第n行的数组
            }
            warshall(matrix).then((TRMatrix) => {
                interfaceShow(TRMatrix); // 显示界面
            }).catch((exception) => {
                stdout.write(`${exception}\n`);
            }).finally(() => {
                resetMatrix(matrix); // 重置矩阵
            });
        } else {
            matrix[rowIndex] = lineString.split(' '); // 矩阵前n - 1行的数组
            rowIndex++;
            stdout.write('> ');
        }
    });
})();
