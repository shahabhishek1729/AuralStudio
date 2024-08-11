#![allow(dead_code)] // Code that is used in the tests module is not recognized here
use regex::{Captures, Regex};
use std::fs;
use std::io::{Read, Write};

/// Creates a wrapper around files stored on the user's system.
/// Only stores files ending in the following extensions:
///  1. '.rattle'
///  2. '.python'
///  3. '.py'
///  File consists of three parts: 'name' (the name of the file,
///  excluding extensions), 'ext' (the file extension) and the
///  'full_name' ('name + ext', stores the entire name of the file
///  including its extension)
///
///  # Examples
///  ```
///  use rattlesnake::file_utils::File;
///
///  let mut file = File::new("abc.py");
///  file.parse();
///  assert_eq!(file.name.unwrap(), "abc");
///  assert_eq!(file.ext.unwrap(), "py");
///  assert_eq!(file.full_name, "abc.py");
///  ```
#[derive(Debug)]
pub struct File {
    ///
    pub full_name: String,
    ///
    pub name: Option<String>,
    ///
    pub ext: Option<&'static str>,
    ///
    pub contents: Option<String>,
}

impl File {
    /// Creates a new File object (from a filename that exists in
    /// the current directory).
    ///
    ///  # Examples
    ///  ```
    ///  use rattlesnake::file_utils::File;
    ///
    ///  let mut file = File::new("abc.py");
    ///  // We now have a file that we can interact
    ///  // with using the methods listed below.
    ///  ```
    pub fn new(name: &str) -> Self {
        return File {
            full_name: String::from(name),
            name: None,
            ext: None,
            contents: None,
        };
    }

    /// Parses the filename into a name and extension. ALl filenames
    /// are expected to be of the form <NAME>.<EXT>, and these values
    /// are populated into the `name` and `ext` fields, respectively.
    ///
    /// # Panics
    /// If the filename is does not match the parsed string '<NAME>.<EXT>'
    ///
    /// # Examples
    /// ```
    /// use rattlesnake::file_utils::File;
    ///
    /// let mut file = File::new("abc.py");
    /// file.parse();
    /// assert_eq!(file.name.unwrap(), "abc");
    /// assert_eq!(file.ext.unwrap(), "py");
    /// ```
    pub fn parse(&mut self) {
        self.parse_name_();
        self.parse_ext_();
        assert_eq!(
            format!(
                "{}.{}",
                self.name
                    .clone()
                    .expect("Name should be parsed by this point"),
                self.ext
                    .clone()
                    .expect("Extension should be parsed by this point")
            ),
            self.full_name
        );
    }

    fn parse_name_(&mut self) {
        let splits = self.full_name.split(".").collect::<Vec<_>>();
        let name = splits[..splits.len() - 1].join(".");
        self.name = Some(name)
    }

    fn parse_ext_(&mut self) {
        let splits = self.full_name.split(".").collect::<Vec<_>>();
        let last = splits.last();
        match last {
            Some(last_) => {
                let static_last = match *last_ {
                    "py" => "py",
                    "python" => "python",
                    "rattle" => "rattle",
                    _ => panic!("Cannot open any type of file other than .py, .python or .rattle"),
                };

                self.ext = Some(static_last);
            }
            None => panic!("Cannot open files without extensions"),
        }
    }

    ///
    pub fn read(&mut self) -> String {
        let mut f = self.to_fs_(false);
        let mut buf = String::new();
        match f.read_to_string(&mut buf) {
            Ok(_) => {
                self.contents = Some(buf.clone());
                return buf;
            }
            Err(msg) => panic!("{}", msg),
        };
    }

    ///
    pub fn write(&mut self, to_write: &str) {
        let mut f = self.to_fs_(true);
        self.contents = Some(String::from(to_write));
        f.write_all(to_write.as_bytes())
            .expect("Could not open file");
    }

    fn to_fs_(&self, write: bool) -> fs::File {
        // Convenience function to quickly cast files into the std::fs
        // File type.
        if write {
            dbg!("CREATING");
            fs::File::create(&self.full_name).expect("Failed to open file")
        } else {
            dbg!("READING");
            fs::File::open(&self.full_name).expect("Failed to open file")
        }
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        // Test if the contents of two files are equal (almost
        // always used to comapre a Rattle script with Python),
        // and returns `true` if the actual contents of the two
        // files are equal. That is, a sequence of spaces followed
        // by a newline should be ignored in the comparison
        let space_newline_re = Regex::new(r"\s+\n").expect("Invalid regex");
        let leading_newline_re = Regex::new(r"^(?:\n)+").expect("Invalid regex");
        let trailing_newline_re = Regex::new(r"(?:\n)*$").expect("Invalid regex");

        let self_src = self.contents.clone().unwrap();
        let orig_replaced1 =
            &*space_newline_re.replace_all(&self_src, |_: &Captures<'_>| String::from("\n"));
        let orig_replaced2 =
            &*leading_newline_re.replace_all(&orig_replaced1, |_: &Captures<'_>| String::from(""));
        let orig_replaced =
            &*trailing_newline_re.replace_all(&orig_replaced2, |_: &Captures<'_>| String::from(""));

        let oth_src = other.contents.clone().unwrap();
        let oth_replaced1 =
            &*space_newline_re.replace_all(&oth_src, |_: &Captures<'_>| String::from("\n"));
        let oth_replaced2 =
            &*leading_newline_re.replace_all(&oth_replaced1, |_: &Captures<'_>| String::from(""));
        let oth_replaced =
            &*trailing_newline_re.replace_all(&oth_replaced2, |_: &Captures<'_>| String::from(""));

        dbg!("replaced, we have:");
        dbg!(&orig_replaced);
        dbg!(&oth_replaced);

        orig_replaced == oth_replaced
    }
}
