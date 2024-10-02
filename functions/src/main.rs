fn fun2() {
    print!("second function \n");
}
fn fun_wth_prms(x: i32) {
    println!("Param function The value of x is :{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
// fn operation(){
//     let y ={
//         let x=3;
//         x+1
//     }
//     print!("The value of y is: {y}");
// }

fn plus_one(z: i32) -> i32 {
    z + 1
}

fn five_plus_one() {
    let z = plus_one(6);
    println!("The value of z is {z}");
}

fn main() {
    println!("Main function!");
    fun2();
    fun_wth_prms(10);
    print_labeled_measurement(5, 'h');
    five_plus_one();
}
