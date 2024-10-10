fn main() {
    let energy = 100; //original owener of the energy value
                      //Immutable borrow -
    let observer1 = &energy; // observer1 borrows energy
    let observer2 = &energy; //overserver2 also borrows energy
    println!("Observer 1 sees energy: {}", observer1);
    println!("Observer 2 sees energy: {}", observer2);
    //Mutable borrow
    let mut machine = 50; //Machine has its own energy level
    {
        let adjustment = &mut machine; // Borrowing own energy level
        *adjustment += 20; //Adjusting the machine's energy.
    } //Adjustment ends here, and `machine` is available again

    println!("Machine's energy after adjustment: {}", machine);
}
