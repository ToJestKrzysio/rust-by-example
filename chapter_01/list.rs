use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (index, value) in vec.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", index, value)?;
        }
        write!(f, "]")
    }
}


fn main() {
    let list = List(vec![1, 2, 3]);

    println!("List");
    println!("Display: {}", list);
    println!("Debug: {:?}", list);
}
