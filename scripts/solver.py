from collections import namedtuple, defaultdict

fields = ["u_start", "u_end", "section_id", "lecture_id", "section_name", "meeting_type"]
SectionMeeting = namedtuple("SectionMeeting", fields)

data = [[135000, 139500, 2023337427, 2023337427, 'CSE-185-01', 'Lecture'], [140400, 144900, 2023337412, 2023337412, 'CSE-150-01', 'Lecture'], [226800, 231300, 2023336415, 2023336415, 'CSE-168-01', 'Lecture'], [237600, 247500, 2023337795, 2023337795, 'CSE-175-01', 'Lecture'], [243000, 253200, 2023337413, 2023337412, 'CSE-150-03L', 'Lab'], [297000, 307200, 2023336684, 2023336415, 'CSE-168-03L', 'Lab'], [307800, 312300, 2023337427, 2023337427, 'CSE-185-01', 'Lecture'], [313200, 317700, 2023337412, 2023337412, 'CSE-150-01', 'Lecture'], [329400, 339600, 2023337429, 2023337427, 'CSE-185-04L', 'Lab'], [399600, 404100, 2023336415, 2023336415, 'CSE-168-01', 'Lecture'], [459000, 468900, 2023337796, 2023337795, 'CSE-175-02L', 'Lab'], [469800, 479700, 2023337797, 2023337795, 'CSE-175-03L', 'Lab'], [480600, 490500, 2023337798, 2023337795, 'CSE-175-04L', 'Lab'], [491400, 501300, 2023337799, 2023337795, 'CSE-175-05L', 'Lab'], [491400, 501600, 2023337414, 2023337412, 'CSE-150-04L', 'Lab'], [892800, 903600, 2023337795, 2023337795, 'CSE-175-01', 'Exam'], [905400, 916200, 2023336415, 2023336415, 'CSE-168-01', 'Exam'], [991800, 1002600, 2023337412, 2023337412, 'CSE-150-01', 'Exam'], [1164600, 1175400, 2023337427, 2023337427, 'CSE-185-01', 'Exam']]

sections = [SectionMeeting(*x) for x in data]

# make sections into lists
labs = defaultdict(list)
lectures = defaultdict(list)
discussions = defaultdict(list)
exams = defaultdict(list)

for section in sections:
    if section.meeting_type == "Lecture":
        lectures[section.lecture_id].append(section)
    elif section.meeting_type == "Lab":
        labs[section.lecture_id].append(section)
    elif section.meeting_type == "Discussion":
        discussions[section.lecture_id].append(section)
    elif section.meeting_type == "Exam":
        exams[section.lecture_id].append(section)

all_courses = [*set(section.lecture_id for section in sections)]

def is_valid(schedule):
    schedule_sorted = sorted(list(schedule), key=lambda x: x.u_start)
    for i in range(1, len(schedule_sorted)):
        if schedule_sorted[i].u_start <= schedule_sorted[i-1].u_end:
            return False
    return True

def add_lectures(target_set, lectures):
    for lecture in lectures:
        target_set.add(lecture)

def remove_lectures(target_set, lectures):
    for lecture in lectures:
        target_set.remove(lecture)

schedule = set()
classes = []
def solve(consider_idx):
    if not is_valid(schedule):
        return
    
    if len(classes) == len(all_courses): 
        for section in schedule:
            print(section)
        print()
        return
    
    if consider_idx == len(all_courses): # not max # of classes
        return
    
    # take the class
    lecture_id = all_courses[consider_idx]
    lecture_meetings = lectures[lecture_id]
    classes.append(lecture_id)
    add_lectures(schedule, lecture_meetings)

    # then pick a lab
    for lab in labs[lecture_id]:
        schedule.add(lab)
        # then pick an exam
        for exam in exams[lecture_id]:
            schedule.add(exam)
            solve(consider_idx+1)
            schedule.remove(exam)
            
        schedule.remove(lab)
    
    # or not take the class
    classes.pop()
    remove_lectures(schedule, lecture_meetings)
    solve(consider_idx+1)

solve(0)
