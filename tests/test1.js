const warshall = require('../src/warshall');
warshall([[1, 0], [0, 1]]).then((matrix) => {
    console.table(matrix);
});