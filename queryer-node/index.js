const { example_sql, query } = require('.')

let sql = example_sql()

console.log(query(sql))

// console.log(query(sql, "json"))
