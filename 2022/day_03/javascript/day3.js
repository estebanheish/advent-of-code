const fs = require('fs')

function rank(c) {
  if (c.toLowerCase() === c) {
    return c.charCodeAt(0) - 97 + 1
  } else {
    return c.charCodeAt(0) - 65 + 27
  }
}

let input = fs.readFileSync('../input.txt', 'utf-8').split('\n')

let out = 0;
for (let e of input) {
  let l = e.length / 2

  let x = new Set(e.slice(0, l))
  let y = e.slice(l, e.length)

  for (let i of x) {
    if (y.includes(i)) {
      out += rank(i)
    }
  }
}
console.log(out)

out = 0;
for (let i = 0; i < input.length; i += 3) {
  let x = new Set(input[i])
  let y = input[i + 1]
  let z = input[i + 2]

  for (let s of x) {
    if (y.includes(s) && z.includes(s)) {
      out += rank(s)
    }
  }
}
console.log(out)

