use std::io;

fn main() {
    #[allow(dead_code)]
    struct ErrorMessages<'a> {
        io_error_message: &'a str,
        exit_code: &'a str,
    }

    const _MAX_VALUE: u64 = 1024;
    println!("* * * Please provide the memory limit * * *");

    let mut input_memory: String = String::new();
    let read_input_error = ErrorMessages {
        io_error_message: "Failed to read input...",
        exit_code: "1",
    };

    io::stdin()
        .read_line(&mut input_memory)
        .expect(&read_input_error.io_error_message);

    println!("Allocated memory -> {}kbs", input_memory);
}
