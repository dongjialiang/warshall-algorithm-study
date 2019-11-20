# Warshall-algorithm-study

## Example
### cli
```powershell
PS E:\ng\Test\test> npx warshall
*要求输入矩阵中每行的元素以空格区分,并且元素以0, 1表示
*输入`.exit`结束
-----------------------------------
请输入矩阵: (结尾写上ok表示结束)
> 0 1 0 0
> 1 0 1 0
> 0 0 0 1
> 0 1 0 0ok
________________________________________
  \ col|1      |2      |3      |4      |
row\   |_______|_______|_______|_______|
1      |1      |1      |1      |1      |
2      |1      |1      |1      |1      |
3      |1      |1      |1      |1      |
4      |1      |1      |1      |1      |
________________________________________
请输入矩阵: (结尾写上ok表示结束)
> .exit
  再见朋友, 欢迎你下次使用~
```
### file
```js
const warshall = require('warshall-algorithm-study');
warshall([[1, 0], [0, 1]]).then((matrix) => {
    console.table(matrix);
});
```
## Install
`npm i warshall-algorithm-study`
## Dev
`npm start`
## Run
`npx warshall`
