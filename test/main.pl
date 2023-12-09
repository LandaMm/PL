
import wheels.Wheel

const wheel = Wheel(WHEEL_PORT)

fn start() {
  wheel.speed = 0
}

fn update(delta) {
  wheel.speed += 1
}
