# Borrowing Analogy with Energy Transfer
```
Key Concepts
1. Energy Transfer: In physics, when energy is transferred from one object to another (like kinetic energy from a moving ball to a stationary one), the original object still retains its energy. In Rust, when a variable is borrowed (using &), the original owner retains ownership of the value while allowing temporary access to it.

```
```
2. Temporary Access: Just as energy can be temporarily transferred for a specific purpose (like powering a device), borrowing allows a function to use a value without taking full ownership. This means the original value remains intact and can still be used after the borrowing ends.

```
```
3. Immutable and Mutable Borrowing: In physics, energy can be transferred in different forms (kinetic vs. potential). Similarly, Rust allows two types of borrowing:

              a) Immutable Borrowing (&T): Multiple references can exist simultaneously, akin to multiple observers measuring the same energy without altering it.

              b) Mutable Borrowing (&mut T): Only one mutable reference can exist at a time, similar to how only one person can adjust a dial on a machine at any given moment.

```
please check main.rs file for example


### Explanation
In this example, the integer 100 represents an initial amount of energy owned by energy. Both observer1 and observer2 borrow this value immutably, allowing them to read its value without modifying it.
The variable machine, which starts with an energy level of 50, is borrowed mutably by adjustment. The value is modified within a scope, demonstrating that while it's borrowed mutably, no other references (mutable or immutable) can exist simultaneously.
After the mutable borrow ends (when the scope is exited), we can safely use machine again.


### Conclusion
Rust's borrowing mechanism ensures safe access to data while maintaining ownership integrity. Just like in physics where energy can be transferred without loss of ownership, Rust allows variables to be borrowed temporarily for use in functions or other contexts. This approach prevents issues like data races and ensures memory safety by enforcing strict rules around how and when data can be accessed or modified.
