# Warshall-algorithm-study

Practice the warshall algorithm

## Example

```powershell
PS E:\ng\Test\test\warshall-backup\rust> .\warshall.exe
*要求输入矩阵中每行的元素以空格区分,并且元素以0, 1表示
*输入`.exit`结束
-----------------------------------

请输入矩阵: (结尾写上ok表示结束)
> 0 1 0 0
> 1 0 1 0
> 0 0 0 1
> 0 1 0 0ok
matrix:
1 1 1 1
1 1 1 1
1 1 1 1
1 1 1 1

请输入矩阵: (结尾写上ok表示结束)
> .exit
 再见朋友, 欢迎下次使用
PS E:\ng\Test\test\warshall-backup\rust>
```
## Build
`rustc .\warshall.rs`

## Run
`./warshall.exe`
