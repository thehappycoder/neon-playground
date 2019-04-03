const mod = require('.')

const sum = (array) => {
  let result = 0

  for (const elem of array) {
    result = result + elem
  }

  return result
}

const array = []

for (let i = 0; i < process.argv[2]; i++) {
  array.push(i)
}

console.log(array.length)

for (let i = 0; i < 10; i++) {
  console.time('js')
  sum(array)
  console.timeEnd('js')
}

for (let i = 0; i < 10; i++) {
  console.time('rust sum')
  mod.sum(array)
  console.timeEnd('rust sum')
}

for (let i = 0; i < 10; i++) {
  console.time('rust sum2')
  mod.sum2(array)
  console.timeEnd('rust sum2')
}