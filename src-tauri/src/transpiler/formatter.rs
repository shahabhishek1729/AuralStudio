use regex::Regex;

trait Numeric {
    fn floor_fn(&self) -> Self
    where
        Self: Sized;
}

impl Numeric for f64 {
    fn floor_fn(&self) -> f64 {
        self.floor()
    }
}

impl Numeric for f32 {
    fn floor_fn(&self) -> f32 {
        self.floor()
    }
}

///
pub trait PyFormatter {
    ///
    fn fmt(&self) -> String;
}

impl PyFormatter for bool {
    fn fmt(&self) -> String {
        let str_bool = self.to_string();
        format!("{}{}", &str_bool[..1].to_uppercase(), &str_bool[1..])
    }
}

impl PyFormatter for String {
    fn fmt(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl<T> PyFormatter for T
where
    T: Numeric + std::fmt::Display + std::cmp::PartialEq,
{
    fn fmt(&self) -> String {
        let nint = self.floor_fn();

        let dotzero = Regex::new(r"\.0*$").unwrap();

        if *self == nint {
            String::from(&*dotzero.replace_all(&self.to_string(), ""))
        } else {
            self.to_string()
        }
    }
}
