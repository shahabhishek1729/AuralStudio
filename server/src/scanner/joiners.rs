// fn collapse_(&mut self) -> Result<(), String> {
//     while !next_eq!(self, "over") && !self.end_reached_() {
//         if next_eq!(self, "\n") {
//             return Err(String::from("Unterminated modifier"));
//         }
//         self.advance_();
//     }
//
//     if self.end_reached_() {
//         return Err(String::from("Unterminated modifier"));
//     }
//
//     self.advance_();
//
//     let substr = self.source[self.start + 1..self.curr - 1]
//         .iter()
//         .fold(String::new(), |acc, s| format!("{}{}", acc, &s[..1]));
//
//     return match self.identifier_(&substr, false) {
//         Ok(_) => Ok(()),
//         Err(msg) => return Err(msg),
//     };
// }
//
// fn camel_(&mut self) -> Result<(), String> {
//     while !next_eq!(self, "over") && !self.end_reached_() {
//         if next_eq!(self, "\n") {
//             return Err(String::from("Unterminated modifier"));
//         }
//         self.advance_();
//     }
//
//     if self.end_reached_() {
//         return Err(String::from("Unterminated modifier"));
//     }
//
//     self.advance_();
//
//     let substr = self.source[self.start + 1..self.curr - 1]
//         .iter()
//         .enumerate()
//         .fold(String::new(), |acc, (i, s)| {
//             if i == 0 {
//                 format!("{}{}", acc, &s.to_lowercase())
//             } else {
//                 format!(
//                     "{}{}{}",
//                     acc,
//                     &s[..1].to_uppercase(),
//                     &s[1..].to_lowercase()
//                 )
//             }
//         });
//
//     return match self.identifier_(&substr, false) {
//         Ok(_) => Ok(()),
//         Err(msg) => return Err(msg),
//     };
// }
//
// fn pascal_(&mut self) -> Result<(), String> {
//     while !next_eq!(self, "over") && !self.end_reached_() {
//         if next_eq!(self, "\n") {
//             return Err(String::from("Unterminated modifier"));
//         }
//         self.advance_();
//     }
//
//     if self.end_reached_() {
//         return Err(String::from("Unterminated modifier"));
//     }
//
//     self.advance_();
//
//     let substr = self.source[self.start + 1..self.curr - 1]
//         .iter()
//         .fold(String::new(), |acc, s| {
//             format!(
//                 "{}{}{}",
//                 acc,
//                 &s[..1].to_uppercase(),
//                 &s[1..].to_lowercase()
//             )
//         });
//
//     return match self.identifier_(&substr, false) {
//         Ok(_) => Ok(()),
//         Err(msg) => return Err(msg),
//     };
// }
//
// fn snake_(&mut self) -> Result<(), String> {
//     while !next_eq!(self, "over") && !self.end_reached_() {
//         if next_eq!(self, "\n") {
//             return Err(String::from("Unterminated modifier"));
//         }
//         self.advance_();
//     }
//
//     if self.end_reached_() {
//         return Err(String::from("Unterminated modifier"));
//     }
//
//     self.advance_();
//
//     let substr = self.source[self.start + 1..self.curr - 1].join("_");
//
//     return match self.identifier_(&substr, false) {
//         Ok(_) => Ok(()),
//         Err(msg) => return Err(msg),
//     };
// }
