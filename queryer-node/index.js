const { example_sql, query } = require('.')

// let sql = example_sql()

let source = 'https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv'
let sql = `\
SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
FROM ${source} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6`

let result = query(sql)

if (result) {
  let rows = result.trim().split('\n')
  let head = rows[0].split(',')
  let body = rows.slice(1).map(r => r.split(','))
  console.table(body.map(r => r.reduce((acc, x, i) => {
    return {
      ...acc,
      [head[i]]: x
    }
  }, {})), head)
} else {
  console.log('Empty Result')
}
