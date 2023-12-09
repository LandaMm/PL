
class Creature {
  kind = "unknown"

  fn init(kind) {
    self.kind = kind
  }

  fn die() {
    print("dying")
  }
}

class Human from Creature {
  age = 1

  fn init(age) {
    super("human")
    self.age = age
  }

  fn birthday() {
    self.age += 1
  }
}

class Person from Human {
  name = null

  fn init(name, age) {
    super(age)
    self.name = name
  }

  # create empty person (with no data)
  static fn default() {
    return Person("unknown", 0)
  }

  fn changeName(newName) {
    self.name = newName
  }
}

const wheel = Wheel(WHEEL_PORT)

fn start() {
  wheel.speed = 0
}

fn update(delta) {
  wheel.speed += 1
}
