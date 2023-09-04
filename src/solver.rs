use std::collections::HashMap;

use crate::parse::{CourseListContext, SectionMeeting};

pub struct CoursePreferences {
    lecture_ids: Vec<SectionMeeting>,

    required_lectures: HashMap<u64, u64>,
    required_discussion: HashMap<u64, bool>,
    required_lab: HashMap<u64, bool>,

    n_courses: u64,
}

pub struct BTSolver {
    prefs: CoursePreferences
}

impl BTSolver {

    // assume schedule is always sorted by start date
    pub fn score(&self, schedule: &Vec<SectionMeeting>) -> f64 {
        // check any intersections
        
        -1.0
    }
}

#[test]
fn modulo_test() {
    println!("{:?}", (-1)%12);
    // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
}