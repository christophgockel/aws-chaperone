use std::io;

pub enum ExecutionMode {
    NoOp,
    Initialize,
}

pub fn initialize(stdout: &mut io::Write) {
    stdout
        .write("Initializing chaperone\n".as_bytes())
        .expect("Something went wrong.");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdoutDouble {
        pub written_content: String,
    }

    impl StdoutDouble {
        fn new() -> StdoutDouble {
            StdoutDouble {
                written_content: String::new(),
            }
        }
    }

    impl io::Write for StdoutDouble {
        fn write(&mut self, content: &[u8]) -> std::result::Result<usize, std::io::Error> {
            self.written_content = std::str::from_utf8(content).unwrap().to_string();

            Ok(0)
        }
        fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    #[test]
    fn prints_initialization_message() {
        let mut stdout = StdoutDouble::new();

        initialize(&mut stdout);

        assert_eq!("Initializing chaperone\n", stdout.written_content)
    }
}
