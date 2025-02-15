fn main() {
    let mut x = 5;

    { //This limits the scope of mutable borrow to avoid the conflict.
        let y = &mut x;
        *y += 1;
    }

    { //This limits the scope of mutable borrow to avoid the conflict.
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}