use std::{collections::{HashMap, HashSet}, hash::Hash, fs};
use log::warn;
use serde_json::Value;

use crate::parse::{CourseListContext, SectionMeeting, MeetingType};

#[derive(Debug)]
pub struct CoursePreferences {
    // 0, 1, 2, 3 are lecture ids for any 4 classes
    lecture_ids: Vec<u64>, 

    // indexed from 0 to whatever as 'internalized' idx for faster lookup and bitmasking
    sections: Vec<SectionMeeting>,

    // grouping by discussion and lab sections sections_idx indexed by lecture_ids' idx
    lab_sections: Vec<Vec<usize>>,
    lecture_sections: Vec<Vec<usize>>,
    discussion_sections: Vec<Vec<usize>>,
    exam_sections: Vec<Option<usize>>
}

pub struct BTSolver {
    prefs: CoursePreferences,
}

impl CoursePreferences {
    pub fn new(lecture_ids: Vec<u64>, course_ctx: CourseListContext) -> Self {
        let mut lab_sections = vec![Vec::new(); lecture_ids.len()];
        let mut discussion_sections = vec![Vec::new(); lecture_ids.len()];
        let mut lecture_sections = vec![Vec::new(); lecture_ids.len()];
        let mut exam_sections = vec![None; lecture_ids.len()];

        let mut lecture_id_to_idx: HashMap<u64, usize> = HashMap::new();
        lecture_ids.iter().enumerate().for_each(|(i, k)| {
            lecture_id_to_idx.insert(*k, i);
        });

        let mut sections = course_ctx.meetings_from_lectures(&lecture_ids);
        sections.sort_by_key(|section| section.u_start);

        sections.iter().enumerate().for_each(|(idx, &section)| {
            let lecture_idx = lecture_id_to_idx[&section.lecture_id];

            match section.meeting_type {
                MeetingType::Lecture => lecture_sections[lecture_idx].push(idx),
                MeetingType::Discussion => discussion_sections[lecture_idx].push(idx),
                MeetingType::Lab => lab_sections[lecture_idx].push(idx),
                MeetingType::Exam => exam_sections[lecture_idx] = Some(idx),
                MeetingType::Other => warn!("building course preferences index OTHER: {:?}", section),
            };
        });

        Self {
            lecture_ids,
            sections,

            lab_sections,
            lecture_sections,
            discussion_sections,
            exam_sections
        }
    }
}

impl BTSolver {
    pub fn new(prefs: CoursePreferences) {

    }

    pub fn solve(&self) {

    }

    fn backtrack(&self, schedule_mask: &Vec<bool>) {
        
    }

    // assume schedule is always sorted by start date
    // assume at least one lecture session for all desired lectures
    // fn score(&self, schedule: &Vec<SectionMeeting>) -> f64 {
    //     // check any intersections and count lectures and discussions
    //     let mut ending = 0; // pretty sure no classes are on sunday 12am
    //     let mut seen_lab = HashSet::new();
    //     let mut seen_discussion = HashSet::new();
    
    //     for i in 0..schedule.len() {
    //         if schedule[i].u_start <= ending {
    //             return -1.0; // a "soft rejection" preferred in GASolver. penalize by overlap in time
    //         }

    //         let lecture_id = schedule[i].lecture_id;

    //         let is_lab = matches!(schedule[i].meeting_type, MeetingType::Lab);
    //         if is_lab && seen_lab.contains(&lecture_id) { return -1.0; } // makes no sense to have 2 lab sections

    //         let is_discussion = matches!(schedule[i].meeting_type, MeetingType::Discussion);
    //         if is_discussion && seen_discussion.contains(&lecture_id) { return -1.0; } // makes no sense to have 2 discussion sections

    //         if is_lab { seen_lab.insert(lecture_id); }
    //         else if is_discussion { seen_discussion.insert(lecture_id); }

    //         ending = schedule[i].u_end;
    //     }
 
    //     // check all the lectures have disc/lab if they require them
    //     if self.prefs.lecture_ids.iter().all(|lecture_id| {
    //         let req_discussion = self.prefs.required_discussion[lecture_id];
    //         let req_lab = self.prefs.required_lab[lecture_id];
    //         (!req_discussion || (req_discussion && seen_discussion.contains(lecture_id))) &&
    //         (!req_lab || (req_lab && seen_discussion.contains(lecture_id)))
    //     }) {
    //         return -1.0; // GASolver "soft rejection" penalizes by the number of missing sections
    //     }

    //     1.0 // again, "encouraging approval" preferred in GA solver, incentivizing "good" schedules like no late classes
    // }
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