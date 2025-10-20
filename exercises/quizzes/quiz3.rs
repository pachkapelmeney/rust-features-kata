// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

use std::{f32, fmt::{Display, Error}, num::ParseIntError, string::ParseError};

// TODO: Adjust the struct as described above.
struct ReportCard<T> 
    where T: Display {
    grade: T,
    student_name: String,
    student_age: u8,
}

// impl<T> Grade<T> for ReportCard<T> {
    
// }

// trait Grade<T> {

// }

// TODO: Adjust the impl block as described above.
impl<T> ReportCard<T> where T: Display{
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

// impl ReportCard {
//     fn print(&self) -> String {
//         format!(
//             "{} ({}) - achieved a grade of {:?}",
//             &self.student_name, &self.student_age, Self::match_grade(&self.grade),
//         )
//     }

//     fn match_grade(grade: &f32) -> Result<String, Error> {
//         match grade {
//             5.0 => Ok("A+".to_string()),
//             4.5=> Ok("A".to_string()),
//             _ => Err(Error)
//         }
//     }
// }

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
