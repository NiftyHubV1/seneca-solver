import json
import os
import requests
import datetime


def get_user_id(access_key):
    url = "https://user-info.app.senecalearning.com/api/user-info/me"

    headers = {
        "Host": "user-info.app.senecalearning.com",
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
        "Accept": "*/*",
        "Accept-Language": "en-GB,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br, zstd",
        "Referer": "https://app.senecalearning.com/",
        "access-key": access_key,
        "Content-Type": "application/json",
        "correlationId": "1737330516472::76115c42-02c9-4d56-0000-000000000000",
        "user-region": "GB",
        "Origin": "https://app.senecalearning.com",
        "DNT": "1",
        "Sec-GPC": "1",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-site",
        "Connection": "keep-alive",
        "host": "user-info.app.senecalearning.com",
    }

    response = requests.request("GET", url, headers=headers)

    if response.status_code == 200:
        return response.json()["userId"]
    else:
        print("Failed to get user id")
        print(response.text)
        exit(code=-1)


def get_signed_url(course_id, section_id, access_key):
    url = f"https://course.app.senecalearning.com/api/courses/{course_id}/signed-url?sectionId={section_id}&contentTypes=standard,hardestQuestions"

    headers = {
        "Host": "course.app.senecalearning.com",
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
        "Accept": "*/*",
        "Accept-Language": "en-GB,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br, zstd",
        "Referer": "https://app.senecalearning.com/",
        "access-key": access_key,
        "Content-Type": "application/json",
        "correlationId": "1737330516472::76115c42-02c9-4d56-0000-000000000000",
        "user-region": "GB",
        "Origin": "https://app.senecalearning.com",
        "DNT": "1",
        "Sec-GPC": "1",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-site",
        "Connection": "keep-alive",
        "host": "course.app.senecalearning.com",
    }

    response = requests.request("GET", url, headers=headers)

    if response.status_code == 200:
        signed_url = response.json().get("url")
        # print("Got signed url")
        return signed_url
    else:
        print("Failed to get signed url")
        print(response.text)
        exit(code=-1)


def get_content(course_id, section_id, access_key):
    url = get_signed_url(course_id, section_id, access_key)

    payload = {}
    headers = {
        "Host": "course-cdn-v2.app.senecalearning.com",
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
        "Accept": "*/*",
        "Accept-Language": "en-GB,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br, zstd",
        "Referer": "https://app.senecalearning.com/",
        "access-key": access_key,
        "Content-Type": "application/json",
        "correlationId": "1737330516472::76115c42-02c9-4d56-0000-000000000000",
        "user-region": "GB",
        "Origin": "https://app.senecalearning.com",
        "DNT": "1",
        "Sec-GPC": "1",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-site",
        "Connection": "keep-alive",
        "host": "course-cdn-v2.app.senecalearning.com",
    }

    response = requests.request("GET", url, headers=headers, data=payload)
    if response.status_code == 200:
        # print("Got content")
        return response.json()
    else:
        print("Failed to get content")
        print(response.text)
        exit(code=-1)


def run_solver(course_id, section_id, content_id, user_id, access_key):
    json_data = get_content(course_id, section_id, access_key)

    def random_hex_string(length=6):
        return os.urandom(length).hex()

    session_id = "b0f80b52-c6d0-42f6-b462-" + random_hex_string(12)

    content = {}

    for cont in json_data["contents"]:
        if cont["id"] == content_id:
            content = cont
            break

    time = datetime.datetime.now(datetime.timezone.utc).replace(microsecond=0)
    time_started = datetime.datetime.now(datetime.timezone.utc).replace(
        microsecond=0
    ) - datetime.timedelta(minutes=2)
    time_started_module = datetime.datetime.now(datetime.timezone.utc).replace(
        microsecond=0
    ) - datetime.timedelta(seconds=15)

    data = {
        "platform": "seneca",
        "clientVersion": "2.13.81",
        "userId": user_id,
        "userLevelFeatureFlagValue": "control",
        "session": {
            "sessionId": session_id,
            "courseId": course_id,
            "timeStarted": time_started.isoformat(),
            "timeFinished": time.isoformat(),
            "startingProficiency": 0,
            "endingProficiency": 0.5,
            "startingCourseProficiency": 0.003601579633505109,
            "endingCourseProficiency": 0.04580470162748644,
            "endingCourseScore": 0.07210750573582432,
            "sessionScore": 1,
            "completed": True,
            "modulesCorrect": len(
                content["contentModules"]
            ),  # len(json_data["moduleIds"]),
            "modulesIncorrect": 0,
            "averageScore": 1,
            "modulesGaveUp": 0,
            "modulesStudied": len(
                content["contentModules"]
            ),  # len(json_data["moduleIds"]),
            "modulesTested": len(
                content["contentModules"]
            ),  # len(json_data["moduleIds"]),
            "sessionType": "adaptive",
            "sectionIds": [section_id],
            "contentIds": [],
            "options": {
                "hasHardestQuestionContent": (
                    True
                    if content.get("contentType", "") == "hardestQuestions"
                    else False
                ),
            },
        },
        "modules": [],
    }

    module_template = {
        "sessionId": session_id,
        "moduleOrder": 0,  # fill in
        "moduleId": "",  # fill in
        "timeStarted": time_started_module.isoformat(),
        "timeFinished": time.isoformat(),
        "gaveUp": False,
        "submitted": True,
        "completed": True,
        "testingActive": True,
        "content": {},
        "score": 1,
        "moduleScore": {
            "score": 1,
        },
        "userAnswer": {},
        "courseId": course_id,
        "sectionId": section_id,
        "contentId": content_id,
    }

    non_questions = 0

    for module_no, module in enumerate(content["contentModules"]):
        module_template_2 = module_template.copy()
        module_template_2["moduleOrder"] = module_no
        module_template_2["moduleId"] = module["id"]

        module_template_2["moduleType"] = module["moduleType"]

        if (
            module["moduleType"] == "concept"
            or module["moduleType"] == "video"
            or module["moduleType"] == "image"
            or module["moduleType"] == "delve"
        ):
            module_template_2["submitted"] = False
            module_template_2["testingActive"] = False
            module_template_2.pop("moduleScore")
            module_template_2.pop("userAnswer")
            module_template_2["score"] = 0
            non_questions += 1
            # data["session"]["modulesCorrect"] -= 1
            # data["session"]["modulesTested"] -= 1
        elif module["moduleType"] == "toggles":
            module_template_2["userAnswer"] = []

        data["modules"].append(module_template_2)

    # data["session"]["modulesStudied"] = len(data["modules"])
    data["session"]["modulesCorrect"] -= non_questions
    data["session"]["modulesTested"] -= non_questions

    url = "https://stats.app.senecalearning.com/api/stats/sessions"

    payload = json.dumps(data)
    # print(payload)
    headers = {
        "Host": "stats.app.senecalearning.com",
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
        "Accept": "*/*",
        "Accept-Language": "en-GB,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br, zstd",
        "Content-Type": "application/json",
        "Referer": "https://app.senecalearning.com/",
        "access-key": access_key,
        "correlationId": "1737330516472::76115c42-02c9-4d56-0000-000000000000",
        "user-region": "GB",
        "Origin": "https://app.senecalearning.com",
        "DNT": "1",
        "Sec-GPC": "1",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-site",
        "Connection": "keep-alive",
        "host": "stats.app.senecalearning.com",
    }

    response = requests.request("POST", url, headers=headers, data=payload)

    if response.status_code == 200:
        pass
        # print("Submitted session successfully\nDone!")
    else:
        print("Got the following error: ")
        print(response.text)
        exit(code=-1)


def get_assignments(access_key):
    time = datetime.datetime.now(datetime.timezone.utc).replace(microsecond=0) - datetime.timedelta(days=30)

    url = f"https://assignments.app.senecalearning.com/api/students/me/assignments?limit=500&date={requests.utils.quote(time.isoformat())}&archived=false"

    headers = {
        "Host": "assignments.app.senecalearning.com",
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0",
        "Accept": "*/*",
        "Accept-Language": "en-GB,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br, zstd",
        "Referer": "https://app.senecalearning.com/",
        "access-key": access_key,
        "correlationId": "1737330516472::76115c42-02c9-4d56-0000-000000000000",
        "Content-Type": "application/json",
        "user-region": "GB",
        "Origin": "https://app.senecalearning.com",
        "DNT": "1",
        "Sec-GPC": "1",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-site",
        "Connection": "keep-alive",
        "host": "assignments.app.senecalearning.com",
    }

    response = requests.request("GET", url, headers=headers)

    if response.status_code == 200:
        print("Got assignments")
        return response.json()["items"]
    else:
        print("Got the following error: ")
        print(response.text)
        exit(code=-1)


def solve_assignments(assignment, user_id, access_key):
    course_id = assignment["spec"]["courseId"]
    section_id_len = len(assignment["spec"]["sectionIds"])

    for j, section_id in enumerate(assignment["spec"]["sectionIds"]):
        contents = get_content(course_id, section_id, access_key)["contents"]

        for content in contents:
            run_solver(course_id, section_id, content["id"], user_id, access_key)

        print(f"Solved in assignment: {j+1}/{section_id_len}")


# Get access key
access_key = input("Enter your access key: ")

# Get user id
user_id = get_user_id(access_key)

assignments = get_assignments(access_key)

now = datetime.datetime.now(datetime.timezone.utc).replace(microsecond=0)

longest_name_len = 0
for assignment in assignments.copy():    
    start_date = datetime.datetime.strptime(assignment["startDate"],'%Y-%m-%dT%H:%M:%S.000Z').replace(tzinfo=datetime.timezone.utc)

    if start_date > now:
        assignments.remove(assignment)
        continue
    
    longest_name_len = max(longest_name_len, len(assignment["name"]))

print("Assignments:")
for i, assignment in enumerate(assignments):
    print(
        f"{i+1}. {assignment['name'] + ' '*(longest_name_len - len(assignment['name']))} - {assignment['status']} - Due: {assignment['dueDate']} - Start: {assignment['startDate']}"
    )

assignment_no = int(input("Enter the assignment number to solve: ")) - 1

while assignment_no < 0 or assignment_no >= len(assignments):
    print("Invalid assignment number")
    assignment_no = int(input("Enter the assignment number to solve: ")) - 1

solve_assignments(assignments[assignment_no], user_id, access_key)

print("Done!")
