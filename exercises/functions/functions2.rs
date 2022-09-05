// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    call_me(72);
}

fn call_me(num: i64) {
    for i in -1..num {
        println!("Ring! Call number {}", i + 1);
    }
}
