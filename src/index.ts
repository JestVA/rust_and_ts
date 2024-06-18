const foo = [1, 2, 3].map((x) => x + 1)
console.log(foo)

enum Color {
  Red,
  Green,
  Blue
}

function printColor(color: Color) {
  switch (color) {
    case Color.Green:
      console.log('green')
      break
    case Color.Blue:
      console.log('blue')
      break
    case Color.Red:
      console.log('red')
      break
  }
}

printColor(Color.Red)

//////////////////////

type Custom = {
  age: number
  name: string
}

type Item = number | string | Custom

function append(items: Item[]) {
  items.push('Hello there!')
}

const items: Item[] = []

append(items)

console.log(items)

const numbers: number[] = []

append(numbers)

// Emotionally bruising moment
console.log(numbers) // now we have a string in a numbers array!!!
