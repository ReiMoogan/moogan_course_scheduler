from collections import namedtuple, defaultdict

fields = ["u_start", "u_end", "section_id", "lecture_id", "section_name", "meeting_type"]
SectionMeeting = namedtuple("SectionMeeting", fields)

data=[[1164600, 1175400, 2023337427, 2023337427, 'CSE-185-01', 'Exam'], [135000, 139500, 2023337427, 2023337427, 'CSE-185-01', 'Lecture'], [307800, 312300, 2023337427, 2023337427, 'CSE-185-01', 'Lecture'], [329400, 339600, 2023337429, 2023337427, 'CSE-185-04L', 'Lab'], [892800, 903600, 2023337795, 2023337795, 'CSE-175-01', 'Exam'], [237600, 247500, 2023337795, 2023337795, 'CSE-175-01', 'Lecture'], [459000, 468900, 2023337796, 2023337795, 'CSE-175-02L', 'Lab'], [469800, 479700, 2023337797, 2023337795, 'CSE-175-03L', 'Lab'], [491400, 501300, 2023337799, 2023337795, 'CSE-175-05L', 'Lab'], [480600, 490500, 2023337798, 2023337795, 'CSE-175-04L', 'Lab'], [905400, 916200, 2023336415, 2023336415, 'CSE-168-01', 'Exam'], [226800, 231300, 2023336415, 2023336415, 'CSE-168-01', 'Lecture'], [399600, 404100, 2023336415, 2023336415, 'CSE-168-01', 'Lecture'], [297000, 307200, 2023336684, 2023336415, 'CSE-168-03L', 'Lab'], [991800, 1002600, 2023337412, 2023337412, 'CSE-150-01', 'Exam'], [140400, 144900, 2023337412, 2023337412, 'CSE-150-01', 'Lecture'], [313200, 317700, 2023337412, 2023337412, 'CSE-150-01', 'Lecture'], [243000, 253200, 2023337413, 2023337412, 'CSE-150-03L', 'Lab'], [491400, 501600, 2023337414, 2023337412, 'CSE-150-04L', 'Lab']]

sections = [SectionMeeting(*x) for x in data]
sections.sort(key=lambda x: x.u_start)

section_to_idx = {v.section_id: k for k, v in enumerate(sections)}
all_courses = [section_to_idx[lecture] for lecture in set(section.lecture_id for section in sections)]

# make sections into lists
labs = [list() for _ in range(len(sections))]
lectures = [list() for _ in range(len(sections))]
discussions = [list() for _ in range(len(sections))]
exams = [list() for _ in range(len(sections))]

for i, section in enumerate(sections):
    lecture_idx = section_to_idx[section.lecture_id]
    if section.meeting_type == "Lecture":
        lectures[lecture_idx].append(i)
    elif section.meeting_type == "Lab":
        labs[lecture_idx].append(i)
    elif section.meeting_type == "Discussion":
        discussions[lecture_idx].append(i)
    elif section.meeting_type == "Exam":
        exams[lecture_idx].append(i)

# schedule is a mask that is already sorted
def is_valid(schedule):
    last_end = 0
    for idx, taken in enumerate(schedule):
        if not taken: continue
        if sections[idx].u_start <= last_end:
            return False
        last_end = sections[idx].u_end
    return True

def set_lectures(schedule, lectures_idxs, value):
    for lecture_idx in lectures_idxs:
        schedule[lecture_idx] = value

schedule = [False]*len(sections)
def solve(consider_idx, classes):
    if not is_valid(schedule):
        return
    
    if classes == len(all_courses): 
        for idx, taken in enumerate(schedule):
            if taken: print(sections[idx])
        print()
        return
    
    if consider_idx == len(all_courses): # not max # of classes
        return
    
    # take the class
    lecture_id = all_courses[consider_idx]
    lecture_meetings = lectures[lecture_id]
    set_lectures(schedule, lecture_meetings, True)

    # then pick a lab
    for lab_idx in labs[lecture_id]:
        schedule[lab_idx] = True
        # then pick an exam
        for exam_idx in exams[lecture_id]:
            schedule[exam_idx] = True
            solve(consider_idx+1, classes+1)
            schedule[exam_idx] = False
        schedule[lab_idx] = False
    
    # or not take the class
    set_lectures(schedule, lecture_meetings, False)
    solve(consider_idx+1, classes)

solve(0, 0)
