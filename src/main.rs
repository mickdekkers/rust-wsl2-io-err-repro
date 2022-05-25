fn main() {
    let string = (0..2000).map(|_| "X").collect::<String>();
    for i in 0..100 {
        println!("{} {}", string, i);
    }
}
