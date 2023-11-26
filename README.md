# Programming Language Parser for "play-to-learn" game

This parser is used for parsing game code and get low-level instructions for game objects in game

## Syntax

Syntax of this PL will be similar to Python syntax

```python
# variables and base types
num = 45 # integer
floating_num = 403.54 # float
my_string = "Hello world!" # string
shop_list = ["Carrot", "Apple", "Milk"] # list
my_obj = { "name": "John", "age": 23 } # object
is_adult = true # boolean

# basic operations
y = 5 + 10 - (3 * 10) - (50 / 10)
y += 56
y -= 12
y *= 5
y /= 10
a = 21 % 10    # a = 1

# functions
def mul(a, b) {
  x = a * b
  return x
}

val = mul(3, 10)   # 30

# if-else statement
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

for i in range(0, 10) {
  a = i
}
```

## API

This programming language is designed specifically for game. It doesn't support any computer API like command line execution, stdout, stdin, stderr and so on

Here are basic API ideas for game objects

Each game object that is programmable will identify as Robot

Robot can have specific tails like wheels, distance to objects detectors, text readers, displays, communication details and so on. All those tails have different API and instructions how to communicate and use them.

Example of using wheels tail with WheelsAPI:

```python
import WheelsAPI.*
import timer

wheels = Wheels.get()

def drive_forward(secs) {
  wheels.speed = 50
  wheels.direction = WheelDirection.FORWARD
  timer.run_and_wait(secs)
}

drive_forward(10) # drive forward for 10 seconds

```

This way all devices (robot tails) will have desired API and documentation. You may noticed that there is timer library that allows developer to create timeout between operations.
