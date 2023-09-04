use std::{collections::{HashMap, HashSet}, hash::Hash, fs};

use serde_json::Value;

use crate::parse::{CourseListContext, SectionMeeting, MeetingType};

#[derive(Debug)]
pub struct CoursePreferences {
    lecture_ids: Vec<u64>,
    section_pool: Vec<SectionMeeting>,

    required_discussion: HashMap<u64, bool>,
    required_lab: HashMap<u64, bool>,
    required_exam: HashMap<u64, bool>,
}

pub struct BTSolver {
    prefs: CoursePreferences
}

impl CoursePreferences {
    pub fn new(lecture_ids: Vec<u64>, course_ctx: CourseListContext) -> Self {
        let mut required_lab = HashMap::new();
        let mut required_discussion = HashMap::new();
        let mut required_exam = HashMap::new();

        let mut section_pool = course_ctx.meetings_from_lectures(&lecture_ids);
        section_pool.sort_by_key(|section| section.u_start);

        section_pool.iter().for_each(|section| {
            required_lab.insert(section.lecture_id, *required_lab.get(&section.lecture_id).unwrap_or(&false) 
                || matches!(section.meeting_type, MeetingType::Lab));
            required_discussion.insert(section.lecture_id, *required_discussion.get(&section.lecture_id).unwrap_or(&false) 
                || matches!(section.meeting_type, MeetingType::Discussion));
            required_exam.insert(section.lecture_id, *required_exam.get(&section.lecture_id).unwrap_or(&false) 
                || matches!(section.meeting_type, MeetingType::Exam));
        });

        Self {
            lecture_ids,
            section_pool,
            required_discussion,
            required_lab,
            required_exam
        }
    }
}

impl BTSolver {
    // assume schedule is always sorted by start date
    // assume at least one lecture session for all desired lectures
    pub fn score(&self, schedule: &Vec<SectionMeeting>) -> f64 {
        // check any intersections and count lectures and discussions
        let mut ending = 0; // pretty sure no classes are on sunday 12am
        let mut seen_lab = HashSet::new();
        let mut seen_discussion = HashSet::new();
    
        for i in 0..schedule.len() {
            if schedule[i].u_start <= ending {
                return -1.0; // a "soft rejection" preferred in GASolver. penalize by overlap in time
            }

            let lecture_id = schedule[i].lecture_id;

            let is_lab = matches!(schedule[i].meeting_type, MeetingType::Lab);
            if is_lab && seen_lab.contains(&lecture_id) { return -1.0; } // makes no sense to have 2 lab sections

            let is_discussion = matches!(schedule[i].meeting_type, MeetingType::Discussion);
            if is_discussion && seen_discussion.contains(&lecture_id) { return -1.0; } // makes no sense to have 2 discussion sections

            if is_lab { seen_lab.insert(lecture_id); }
            else if is_discussion { seen_discussion.insert(lecture_id); }

            ending = schedule[i].u_end;
        }
 
        // check all the lectures have disc/lab if they require them
        if self.prefs.lecture_ids.iter().all(|lecture_id| {
            let req_discussion = self.prefs.required_discussion[lecture_id];
            let req_lab = self.prefs.required_lab[lecture_id];
            (!req_discussion || (req_discussion && seen_discussion.contains(lecture_id))) &&
            (!req_lab || (req_lab && seen_discussion.contains(lecture_id)))
        }) {
            return -1.0; // GASolver "soft rejection" penalizes by the number of missing sections
        }

        1.0 // again, "encouraging approval" preferred in GA solver, incentivizing "good" schedules like no late classes
    }
}

#[test]
fn score_1_class() {
    let res = fs::read("data/mess.json");
    let gql_response: Value = serde_json::from_str(std::str::from_utf8(&res.unwrap()).unwrap()).unwrap();
    let want = vec![2023337427];
    let ctx = CourseListContext::new(&gql_response);
    let prefs = CoursePreferences::new(want, ctx);
    println!("{:?}", prefs)
    // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
}

fn score_2_class_conflict() {
    println!("{:?}", (-1)%12);
    // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
}