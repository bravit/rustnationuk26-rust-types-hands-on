// Goal: Use higher-ranked trait bounds (HRTB) to allow visitors with different lifetimes

trait Visitor<T> {
    fn visit(&self, value: T);
}

struct Printer;
struct Handler;

impl<'a> Visitor<&'a i32> for Printer {
    fn visit(&self, v: &'a i32) {
        println!("Printing {v}");
    }
}

impl<'a> Visitor<&'a i32> for Handler {
    fn visit(&self, v: &'a i32) {
        println!("Handling {v}");
    }
}

// TODO Fix the signature of `process_data` to make the code work
fn process_data<V>(arg: V) {
    let local_data = 10;
    arg.visit(&local_data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        process_data(Printer);
        process_data(Handler);
    }
}
