use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the JavaScript file
    #[arg(name = "script.js")]
    pub file: Option<String>,

    /// Evaluate JavaScript code and print result
    #[arg(short, long, value_name = "...")]
    pub print: Option<String>,
}

impl Args {
    pub fn script(&self) -> String {
        if let Some(file) = &self.file {
            fs::read_to_string(file).unwrap()
        } else if let Some(print) = &self.print {
            print.clone()
        } else {
            panic!(
                "No JavaScript code provided. Please provide JavaScript file or inline script with --print option."
            );
        }
    }
}

#[cfg(test)]
mod tests {
    mod script {
        use super::super::*;
        use std::io::Write;

        #[test]
        fn returns_file_content_when_file_is_provided() {
            // given
            let mut temp_file = tempfile::NamedTempFile::new().unwrap();
            let expected = "console.log('hello');\n";
            writeln!(temp_file, "{}", expected.replace("\n", "")).unwrap();
            let args = Args {
                file: Some(temp_file.path().to_str().unwrap().to_string()),
                print: None,
            };

            // when
            let actual = args.script();

            // then
            assert_eq!(actual, expected);
        }

        #[test]
        fn returns_print_content_when_print_is_provided() {
            // given
            let expected = "1 + 2";
            let args = Args {
                file: None,
                print: Some(expected.to_string()),
            };

            // when
            let actual = args.script();

            // then
            assert_eq!(actual, expected);
        }

        #[test]
        fn prefers_file_over_print() {
            // given
            let mut temp_file = tempfile::NamedTempFile::new().unwrap();
            let expected = "from file\n";
            writeln!(temp_file, "{}", expected.replace("\n", "")).unwrap();
            let args = Args {
                file: Some(temp_file.path().to_str().unwrap().to_string()),
                print: Some("from print".to_string()),
            };

            // when
            let actual = args.script();

            // then
            assert_eq!(actual, expected);
        }

        #[test]
        #[should_panic(
            expected = "No JavaScript code provided. Please provide JavaScript file or inline script with --print option."
        )]
        fn panics_when_neither_file_nor_print_is_provided() {
            // given
            let args = Args {
                file: None,
                print: None,
            };

            // when
            args.script();
        }
    }
}
