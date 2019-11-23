'use strict';
const warshall = async (matrix) => { // warshall算法的函数
    const TRMatrix = [];
    const N = matrix.length;
    for (let i = 0; i < N; i++) { // 检查矩阵的行数和列数是否匹配
        if (matrix[i].length !== N) {
            throw '异常: 行数和列数不匹配!';
        }
        TRMatrix[i] = [];
        for (let j = 0; j < N; j++) {
            TRMatrix[i][j] = matrix[i][j] | 0;
        }
    }
    for (let k = 0; k < N; k++) { // 关系矩阵M^k
        // console.table(TRMatrix); // 取消该行注释可观察每一个M^k的变化
        for (let i = 0; i < N; i++) { // 关系矩阵M的行
            if (TRMatrix[i][k]) {
                /*
                 *  当a(i, k)有一条边时, 如果a(k, j)也有一条边,
                 *  那么根据转递闭包的规则, a(i, j)也应该有一条边
                 *  该选择条件语句的效果相当于矩阵中每一个元素执行:
                 *  TRMatrix[i][j] = (TRMatrix[i][k] && TRMatrix[k][j])
                 *                  ? 1
                 *                  : TRMatrix[i][j];
                 */
                for (let j = 0; j < N; j++) { // 关系矩阵M的列
                    TRMatrix[i][j] = await (TRMatrix[i][j] | TRMatrix[k][j]);
                }
            }
        }
    }
    return TRMatrix;
}
(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? module.exports = factory :
    typeof define === 'function' && define.amd ? define(factory) :
    (global.libName = factory);
}(this, (warshall)));
