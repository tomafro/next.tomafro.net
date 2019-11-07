
trait Length {
    fn length(&self) -> usize;
}

impl Length for &String {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for &Vec<u32> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for &[u32] {
    fn length(&self) -> usize {
        self.len()
    }
}

fn show_length(object: impl Length) {
    println!("Length of {}", object.length())
}

fn main() {
    let a: String = "hello".to_string();
    let b: Vec<u32> = vec![1, 2, 3, 4];

    show_length(&a);
    show_length(&b);

    let c = &b[0..=1];

    show_length(c);
}
