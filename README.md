# Interpreter in Rust
This is a simple interpreter written following the ["Crafting Interpreters"](https://craftinginterpreters.com/) book.

With this language (which does not yet have a name), you can write programs using functions and classes. The classes also support inheritance.
It is not very fast because it is a tree-walk interpreter: this methodology is fine for declarative and high-level languages, but obviously it is not conducive to performance.

## Code examples
Examples of code can be found in the tests/testcases folder where I have written simple programs to test the functionality of the program.
The comments _"--- Test"_ and _"--- Expected"_ are for the purpose of being able to run unit tests via the custom code found in _tests/integration_test.rs_

If you're too lazy to go looking in the code for script examples then I'll leave a couple of very simple examples here.

This is a recursive function that returns the nth number of the fibonacci sequence.
``` kotlin
fun fib(n) {
    if (n <= 1) {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}

print fib(10);


// OUTPUT
// 55
```

This is a simple piece of code for class inheritance.
``` kotlin
class Person {
    init ( name, surname ) {
        this.name = name;
        this.surname = surname;
    }

    print_infos() {
        print "name: " + this.name + " surname: " + this.surname;
    }
}

class Student: Person {
    init( name, surname, course ) {
        this.course = course;
        super.init(name, surname);
    }

    print_infos() {
        print "name: " + this.name + " surname: " + this.surname + " course: " + this.course;
    }
}

var p = Person("Daniel", "Arduini");
p.print_infos();

var s = Student("Daniel", "Arduini", "Nothing");
s.print_infos();


// OUTPUT
// name: Daniel surname: Arduini
// name: Daniel surname: Arduini course: Nothing
```
