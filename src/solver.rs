use std::collections::HashMap;
use log::warn;

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
    sentinel_idx: usize,

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

        sections.iter().enumerate().for_each(|(idx, section)| {
            let lecture_idx = lecture_id_to_idx[&section.lecture_id];

            match section.meeting_type {
                MeetingType::Lecture => lecture_sections[lecture_idx].push(idx),
                MeetingType::Discussion => discussion_sections[lecture_idx].push(idx),
                MeetingType::Lab => lab_sections[lecture_idx].push(idx),
                MeetingType::Exam => exam_sections[lecture_idx] = Some(idx),
                MeetingType::Other => warn!("building course preferences index OTHER: {:?}", section),
            };
        });

        let sentinel_idx = sections.len();
        // this is for courses without lab or discussion or exam. just give a sentinel idx past sections' size 
        for lecture_idx in 0..lecture_ids.len() {
            if lab_sections[lecture_idx].len() == 0 {
                lab_sections[lecture_idx].push(sentinel_idx);
            }
    
            if discussion_sections[lecture_idx].len() == 0 {
                discussion_sections[lecture_idx].push(sentinel_idx);
            }
        }

        Self {
            lecture_ids,
            sections,

            lab_sections,
            lecture_sections,
            discussion_sections,
            sentinel_idx,

            exam_sections
        }
    }
}

fn mask_values(indices: &Vec<usize>, schedule_mask: &mut Vec<bool>, val: bool) {
    indices.iter().for_each(|&idx| {
        schedule_mask[idx] = val;
    });
}

impl BTSolver {
    pub fn new(prefs: CoursePreferences) -> Self {
        BTSolver {
            prefs
        }
    }

    pub fn solve(&self) -> Vec<Vec<SectionMeeting>> {
        let mut schedule_mask = vec![false; self.prefs.sections.len()];
        let mut solutions = Vec::new();

        self.search(&mut solutions, 0, 0, &mut schedule_mask);
        
        solutions
    }

    fn search(&self, solutions: &mut Vec<Vec<SectionMeeting>>, n_added: usize, n_considered: usize, schedule_mask: &mut Vec<bool>) -> usize {
        if n_added == self.prefs.lecture_ids.len() { 
            let mut valid_sections = Vec::new();
            schedule_mask.iter().enumerate().for_each(|(idx, &val)| {
                if val {
                    valid_sections.push(self.prefs.sections[idx].clone());
                }
            });

            solutions.push(valid_sections);
            return n_added; 
        }
        
        if n_considered == self.prefs.lecture_ids.len() { return n_added; }

        // add the class and add all of its lecture sections
        mask_values(&self.prefs.lecture_sections[n_added], schedule_mask, true);
        // exam section
        if let Some(exam_idx) = self.prefs.exam_sections[n_added] {
            schedule_mask[exam_idx] = true;
        }

        for lab_idx in &self.prefs.lab_sections[n_added] {
            // purpose of sentinel index is to let classes without lab or discussion be considered
            if *lab_idx != self.prefs.sentinel_idx {
                // take the lab
                schedule_mask[*lab_idx] = true;
                if self.score(&schedule_mask) < 0.0 { // check early instead of backtrack
                    schedule_mask[*lab_idx] = false;
                    continue; 
                }
            }

            for discussion_idx in &self.prefs.discussion_sections[n_added] {
                if *discussion_idx != self.prefs.sentinel_idx {
                    schedule_mask[*discussion_idx] = true;
                    if self.score(&schedule_mask) < 0.0 { // check early 
                        schedule_mask[*discussion_idx] = false;
                        continue; 
                    }
                }

                // recurse
                self.search(solutions, n_added+1, n_considered+1, schedule_mask);

                if *discussion_idx != self.prefs.sentinel_idx { schedule_mask[*discussion_idx] = false; }
            }

            if *lab_idx != self.prefs.sentinel_idx { schedule_mask[*lab_idx] = false; }
        }

        // or don't add it
        mask_values(&self.prefs.lecture_sections[n_added], schedule_mask, false);
        if let Some(exam_idx) = self.prefs.exam_sections[n_added] {
            schedule_mask[exam_idx] = false;
        }

        // // recurse, missing one course
        self.search(solutions, n_added, n_considered+1, schedule_mask);
    

        return n_added;
    }

    // assume schedule is always sorted by start date
    // the purpose of returning a float is to allow incentives for user preferences
    // probably will add a feature like this if this scheduler actually gets used
    fn score(&self, schedule_mask: &Vec<bool>) -> f64 {
        let mut ending = 0; // pretty sure no classes start sunday 12am
        for section_idx in 0..schedule_mask.len() {
            if !schedule_mask[section_idx] { continue; }
            let section = &self.prefs.sections[section_idx];
            if section.u_start <= ending { return -1.0; }

            ending = section.u_end;
        }

        1.0
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::parse::CourseListContext;
    use crate::solver::{BTSolver, CoursePreferences};
    
    #[test]
    fn score_1_class() {
        let res = fs::read("data/mess.json");
        let gql_response = serde_json::from_str(std::str::from_utf8(&res.unwrap()).unwrap()).unwrap();
        let want = vec![2023337427];
        let ctx = CourseListContext::new(&gql_response);
        let prefs = CoursePreferences::new(want, ctx);
        let solver = BTSolver::new(prefs);
        let res = solver.solve();
        res.iter().for_each(|v| { println!("{:?}", v) });
        // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
    }

    #[test]
    fn score_my_schedule() {
        let res = fs::read("data/mess.json");
        let gql_response = serde_json::from_str(std::str::from_utf8(&res.unwrap()).unwrap()).unwrap();
        let want = vec![2023337427];
        let ctx = CourseListContext::new(&gql_response);
        let prefs = CoursePreferences::new(want, ctx);
        let solver = BTSolver::new(prefs);
        let res = solver.solve();
        res.iter().for_each(|v| { println!("{:?}", v) });
        // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
    }

    #[test]
    fn my_schedule_test() {
        let res = fs::read("data/mess.json");
        let gql_response = serde_json::from_str(std::str::from_utf8(&res.unwrap()).unwrap()).unwrap();
        let want = vec![2023337427, 2023337795,  2023336415, 2023337412];
        let ctx = CourseListContext::new(&gql_response);
        let prefs = CoursePreferences::new(want, ctx);
        // prefs.sections.iter().for_each(|a| { println!("{:?}", a) });
        let solver = BTSolver::new(prefs);
        let res = solver.solve();
        res.iter().for_each(|v| { 
            v.iter().for_each(|a| { println!("{:?}", a) });
            println!();
        });
        // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
    }

    // #[test]
    // fn score_ess001_fail() {
    //     let res = fs::read("data/mess.json");
    //     let gql_response = serde_json::from_str(std::str::from_utf8(&res.unwrap()).unwrap()).unwrap();
    //     let want = vec![2023333322];
    //     todo!("fix this");
    //     let ctx = CourseListContext::new(&gql_response);
    //     let prefs = CoursePreferences::new(want, ctx);
    //     let solver = BTSolver::new(prefs);
    //     let res = solver.solve();
    //     res.iter().for_each(|v| { println!("{:?}", v) });
    //     // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
    // }
}
