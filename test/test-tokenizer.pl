# variables and base types
let num = 45 # integer
let floating_num = 403.54 # float
let my_string = "Hello world!" # string
let shop_list = ["Carrot", "Apple", "Milk"] # list
let my_obj = { "name": "John", "age": 23 } # object
let is_adult = true # boolean

# constants
const PI = 3.1415
const IS_DARWIN = False

const browserWindow = null

# basic operations
let y = 5 + 10 - (3 * 10) - (50 / 10)
y += 56
y -= 12
y *= 5
y /= 10
let a = 21 % 10    # a = 1

# functions
def mul(a, b) {
  x = a * b
  return x
}

let val = mul(3, 10)   # 30

# if-else statement
let y
let t

if x == a and a != 0 {
  y = x
} else if a > 0 or a == -5 {
  y = -x
} else {
  y = 0
  y += a
}

if is_true {
  t = 1
} else {
  t = 0
}

# for-loops
for i in ["first", "second", "third"] {
  y = i
}

let a

for i in range(0, 10) {
  a = i
}

# classes

class Creature {
  init(kind) {
    self.kind = kind
  }

  die() {
    print("dying")
  }
}

class Human from Creature {
  init(age) {
    super("human")
    self.age = age
  }

  birthday() {
    self.age += 1
  }
}

class Person from Human {
  init(name, age) {
    super(age)
    self.name = name
  }

  # create empty person (with no data)
  static default() {
    return new Person("unknown", 0)
  }

  changeName(newName) {
    self.name = newName
  }
}

const jeff = new Person("John", 24)
jeff.changeName("Jeff")
jeff.birthday()
jeff.die()
