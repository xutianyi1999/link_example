#[link(name = "testlib", kind = "static")]
extern "C" {
    fn sum(a: i32, b: i32) -> i32;
}

fn main() {
    let res = unsafe {
        sum(1, 2)
    };
    println!("{}", res)
}