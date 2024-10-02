// To illustrate Rust's ownership model using a physics law analogy, consider the concept of conservation of mass in physics, which states that mass cannot be created or destroyed in an isolated system. This principle can be compared to Rust's ownership rules, where each value in a program has a single owner at any given time, and ownership can be transferred bu
//Ownership Analogy with Conservation of Mass
// Key Concepts
//1 Single Ownership: Just as a physical object cannot exist in two places at once without changing its state, a value in Rust can only have one owner. When ownership is transferred, the previous owner no longer has access to the value, similar to how an object cannot be in two locations simultaneously.
//2 Transfer of Ownership: In physics, when an object is moved from one location to another, it retains its mass but changes its position. In Rust, when a variable is assigned to another variable (e.g., let b = a;), the ownership of the value is transferred from a to b. After this transfer, a becomes invalid (like an object that has been moved), and any attempt to use a will result in a compile-time error.
//3 Scope and Lifetime: The concept of scope in Rust can be likened to the boundaries of a physical system. Just as mass remains within the confines of a closed system until acted upon, values in Rust remain valid within their defined scope. When a variable goes out of scope, its associated memory is automatically released (or "dropped"), akin to how energy or mass dissipates when it exits a closed system.
fn main() {
    let mass = 10; //mass is owened by variable mass
    let new_mass = mass; //owenership is transferred to `new_mass`
                         // println!("Mass: {}", mass); // This line would cause an error because `mass` is no longer valid.
    println!("New Mass: {}", new_mass); // This works because `new_mass` owns the value.
}
