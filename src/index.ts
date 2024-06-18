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
