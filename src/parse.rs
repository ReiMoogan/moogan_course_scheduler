use std::{str, collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::SolveError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MeetingType {
    Lecture, // discussion, lab. some lectures need both lab and discussion
    Discussion,
    Lab,
    Exam,
    Other
}

#[derive(Debug)]
pub struct CourseListContext<'a> {
    id_to_meet_string: HashMap<u64, String>,
    pub id_to_course: HashMap<u64, &'a Value>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SectionMeeting {
    pub u_start: u64,
    pub u_end: u64,
    pub section_id: u64,
    pub lecture_id: u64,
    pub section_name: String,
    pub meeting_type: MeetingType
}

impl From<&str> for MeetingType {
    fn from(value: &str) -> Self {
        match value {
            "Lecture" => Self::Lecture, 
            "Lab" => Self::Lab,
            "Discussion" => Self::Discussion,
            "Exam" => Self::Exam,
            _ => Self::Other
        }
    }
}

fn day_to_str(day: u64) -> &'static str{
    return match day {
        0 => "SAT",
        1 => "MON",
        2 => "TUE",
        3 => "WED",
        4 => "THR",
        5 => "FRI",
        6 => "SUN",
        7 => "EX_SAT",
        8 => "EX_MON",
        9 => "EX_TUE",
        10 => "EX_WED",
        11 => "EX_THR",
        12 => "EX_FRI",
        13 => "EX_SUN",
        _ => "INVALID"
    }
}

fn hour_to_murican(u_time: u64) -> String {
    let hour = u_time % (24 * 3600) / 3600;
    let am_or_pm = if hour >= 12 { "PM" } else { "AM" };
    let hour = if hour == 0 { 12 } else { hour };
    let minute =  u_time % (24 * 3600) % 3600 / 60;
    return format!("{}:{:0>2} {}", hour, minute, am_or_pm);
}

impl Debug for SectionMeeting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_day = self.u_start / (24 * 3600);

        let end_day = self.u_end / (24 * 3600);

        write!(f, "{} {{ start: {} {}, end: {} {}, type: {:?} }}", self.section_name, 
            day_to_str(start_day), hour_to_murican(self.u_start),
            day_to_str(end_day), hour_to_murican(self.u_end), 
            self.meeting_type)
    }
}


impl<'a> CourseListContext<'a> {
    pub fn new(gql_response_json: &'a Value) -> Result<Self, SolveError> {
        let mut id_to_meet_string = HashMap::new();
        let mut id_to_course: HashMap<u64, &'a Value> = HashMap::new();

        gql_response_json["meetingTypes"].as_array()
            .ok_or("invalid meetingTypes")?
            .iter().try_for_each(|val| -> Result<(), SolveError> {
                id_to_meet_string.insert(val["id"].as_u64().ok_or("invalid id")?, val["name"].as_str().ok_or("invalid name")?.into());
                Ok(())
            })?;

        gql_response_json["classes"]["nodes"].as_array()
            .ok_or("invalid nodes")?
            .iter().try_for_each(|val| -> Result<(), SolveError> {
                id_to_course.insert(val["id"].as_u64().ok_or("invalid id")?, val);
                Ok(())
            })?;

        Ok(Self {
            id_to_meet_string,
            id_to_course,
        })
    }

    // generates list of meeting times (of lectures, labs, discussions) from lecture sessions
    pub fn meetings_from_lectures(&self, lecture_ids: &Vec<u64>) -> Result<Vec<SectionMeeting>, SolveError> {
        let mut meetings = Vec::new();

        lecture_ids.iter().try_for_each(|lecture_id| -> Result<(), SolveError> {
            // to cover the main lecture section itself
            self.push_meetings(&mut meetings, *lecture_id, *lecture_id)?;
            // to cover each of its labs/discussion whatever
            self.id_to_course.get(&lecture_id).ok_or("lecture id is not in id_to_course")?["linkedSections"].as_array()
                .ok_or("invalid linkedSections")?
                .iter().try_for_each(|linked_section| -> Result<(), SolveError> {
                    let section_id = linked_section["parent"].as_u64().ok_or("section id doesn't have linked parent")?;
                    self.push_meetings(&mut meetings, *lecture_id, section_id)?;

                    Ok(())
                })?;
            Ok(())
        })?;

        Ok(meetings)
    }

    fn push_meetings(&self, meetings: &mut Vec<SectionMeeting>, lecture_id: u64, section_id: u64) -> Result<(), SolveError>{
        self.id_to_course.get(&section_id).ok_or("section id is not in id_to_course")?["meetings"]
        .as_array().ok_or("meetings not in id_to_course[sectiond_id]")?
        .iter().try_for_each(|meeting_value| -> Result<(), SolveError> {
            let weekday_bits = meeting_value["inSession"].as_u64().ok_or("invalid inSession")?;
            let meeting_id = meeting_value["meetingType"].as_u64().ok_or("invalid meetingType")?;
            let hour_start = meeting_value["beginTime"].as_str().ok_or("invalid beginTime")?;
            let hour_end = meeting_value["endTime"].as_str().ok_or("invalid endTime")?;

            let meeting_type = self.meeting_type_from_id(meeting_id);

            for i in 0..8 {
                if weekday_bits & (1 << i) > 0 {
                    // convert hours to uint
                    let start_digits = hour_start.parse::<u64>().ok().ok_or("cannot parse start_digits")?;
                    let end_digits = hour_end.parse::<u64>().ok().ok_or("cannot parse end_digits")?;
                    // 1130 split into 11 hours 30 mins
                    let mut u_start = i * 24 * 3600 + (start_digits / 100) * 3600 + (start_digits % 100)*60;
                    let mut u_end = i * 24 * 3600 + (end_digits / 100) * 3600 + (end_digits % 100)*60;

                    // make final exams "next week"
                    if matches!(meeting_type, MeetingType::Exam) {
                        u_start += 7 * 24 * 3600;
                        u_end += 7 * 24 * 3600;
                    }

                    meetings.push(SectionMeeting { u_start: u_start, u_end: u_end, 
                        section_id: section_id, lecture_id: lecture_id, meeting_type: meeting_type, 
                        section_name: self.id_to_course.get(&section_id)
                            .ok_or("section id not in id_to_course")?["courseNumber"].to_string() })
                }
            }
            Ok(())
        })?;

        Ok(())
    }

    fn meeting_type_from_id(&self, id: u64) -> MeetingType{
        let meet_string = &*self.id_to_meet_string[&id];
        MeetingType::from(meet_string)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::Value;
    use crate::parse::CourseListContext;
    
    #[test]
    fn parse_test() {
        let res = fs::read("data/mess.json");
        let v: Value = serde_json::from_str(std::str::from_utf8(&res.unwrap()).unwrap()).unwrap();
        let want = vec![2023337427, 2023337795,  2023336415, 2023337412];
        let course_ctx = CourseListContext::new(&v).unwrap();
        let meetings = course_ctx.meetings_from_lectures(&want);
        meetings.iter().for_each(|v| { println!("{:?}", v) });
        // println!("{:?}", v["data"]["classes"]["nodes"].as_array().unwrap().len())
    }
}
