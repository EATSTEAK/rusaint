# rusaint-cli란?

`rusaint-cli`는 cli 환경에서 `rusaint`를 사용하여 강의 정보를 가져와 json 파일로 만들어주는 Rust 바이너리입니다.

# install rusaint-cli

`rusaint-cli` 설치를 위해 먼저 Rust 설치가 선행되어야 합니다. 

아래 명령어를 터미널에 입력하여 `rustup`(러스트 버전 관리 툴)을 설치해줍니다.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`rustup`의 설치가 완료되었다면 아래 명령어를 터미널에 입력해 `rusaint-cli` 바이너리를 설치해줍니다.

```bash
cargo install rusaint-cli
```

## 환경 변수 설정

유세인트에서 강의 정보를 가져오기 위해 유세인트 아이디(학번)와 비밀번호를 환경 변수로 설정해야 합니다.

`rusaint-cli`를 실행할 위치에 아래와 같은 형식의 `.env` 파일을 만들어줍니다.

```
SSO_ID={학번} // 20203123
SSO_PASSWORD={비밀번호} // 1q2w1q2w!
```

# rusaint-cli commands

`rusaint-cli` 는 유세인트 강의시간표 분류에 대응하는 다양한 command가 존재합니다.

## find-major (학부전공)

- 학부전공별 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-major --year <YEAR> --semester <SEMESTER> --college <COLLEGE> --department <DEPARTMENT> --major <MAJOR>
```

### Examples
- 세부 전공이 있는 경우
```bash
cargo run -- find-major --year 2025 --semester 1 --college "공과대학" --department "건축학부" --major "건축공학전공" 
```
- 세부 전공이 없는 경우

```bash
rusaint-cli find-major --year 2025 --semester 1 --college "IT대학" --department "컴퓨터학부"
```

```json
[
  {
    "syllabus": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.HAS_PLAN_PAPER_SALV_WD_CE.1",
    "category": "전선-컴퓨터",
    "sub_category": "복선-컴퓨터",
    "abeek_info": null,
    "field": null,
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "(공통)물리1및실험",
    "division": null,
    "professor": "김남미\n이재구",
    "department": "물리학과",
    "time_points": "4.0/3.0",
    "personeel": "0",
    "remaining_seats": "25",
    "schedule_room": "목 13:00-13:50 (숭덕경상관 02317-김남미)\n목 14:00-14:50 (숭덕경상관 02317-김남미)\n목 15:00-15:50 (조만식기념관 12123-이재구)\n목 16:00-16:50 (조만식기념관 12123-이재구)",
    "target": "전체학년 기계,화공,전기,건축학부,신소재,정통전,전자정보공학부-IT융합,전자정보공학부-전자공학,AI융합,물리,화학,의생명,소프트,컴퓨터"
  },
]
```

## find-required-elective (교양필수)

- 교양필수 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-required-elective --year <YEAR> --semester <SEMESTER> --course-name <COURSE_NAME>
```

### Examples

```bash
rusaint-cli find-required-elective --year 2025 --semester 1 --course-name "대학한국어1"
```

```json
[
  {
    "syllabus": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.HAS_PLAN_PAPER_SALV_WD_CE.1",
    "category": "교필",
    "sub_category": null,
    "abeek_info": null,
    "field": null,
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "대학한국어1",
    "division": null,
    "professor": "김수은",
    "department": "교양교육운영팀",
    "time_points": "6.0/6.0",
    "personeel": "0",
    "remaining_seats": "0",
    "schedule_room": "월 목 09:00-10:15 (진리관 11115-김수은)\n월 목 10:30-11:45 (진리관 11115-김수은)",
    "target": "전체학년 ;순수외국인입학생 (대상외수강제한)(대상외수강제한)"
  },
]
```

## find-optional-elective (교양선택)

- 교양선택 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-optional-elective --year <YEAR> --semester <SEMESTER> --course-name <COURSE_NAME>
```

### Examples

```bash
rusaint-cli find-optional-elective --year 2025 --semester 1 --course-name "[‘23이후]과학·기술" 
```

```json
[
  {
    "syllabus": null,
    "category": "교선",
    "sub_category": null,
    "abeek_info": null,
    "field": "[‘23이후]과학·기술\n['20,'21~'22]창의/융합,균형교양-자연과학·공학·기술\n['19]균형교양-자연/공학(자연/과학/기술)\n['16-'18]기초역량(과학정보기술-정보기술)\n['15이전]정보와기술(융합-자연)",
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "4차산업혁명시대의정보보안",
    "division": null,
    "professor": "장의진",
    "department": "SW교육팀",
    "time_points": "3.0/3.0",
    "personeel": "0",
    "remaining_seats": "50",
    "schedule_room": "화 09:00-10:15 (벤처중소기업센터 10309 (이도영강의실)-장의진)",
    "target": "IT융합전공 ,컴퓨터 ,소프트 ,AI융합학부 ,글로벌미디어 ,정보보호(계약), 교류학과"
  },
]
```

## find-chapel (채플)

- 채플 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-chapel --year <YEAR> --semester <SEMESTER> --chapel-name <CHAPEL_NAME>
```

### Examples

```bash
rusaint-cli find-chapel --year 2025 --semester 1 --chapel-name "비전채플"
```

```json
[
  {
    "syllabus": null,
    "category": "채플",
    "sub_category": null,
    "abeek_info": null,
    "field": null,
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "비전채플",
    "division": null,
    "professor": "김회권",
    "department": "학원선교팀",
    "time_points": "1.0/0.5",
    "personeel": "0",
    "remaining_seats": "980",
    "schedule_room": "화 16:30-17:20 (한경직기념관 08110-김회권)",
    "target": "1학년 내국인학생 전체 수강제한 // 전체학년 수강 신청일: 2학년 이상 전체 단과대학 학생(1학년 외국인유학생 포함) 수강신청 가능"
  },
]
```

## find-education (교직)

- 교직 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-education --year <YEAR> --semester <SEMESTER>
```

### Examples

```bash
rusaint-cli find-education --year 2025 --semester 1
```

```json
[
  {
    "syllabus": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.HAS_PLAN_PAPER_SALV_WD_CE.1",
    "category": "교직",
    "sub_category": null,
    "abeek_info": null,
    "field": "교직이론영역",
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "교육과정",
    "division": null,
    "professor": "조호제",
    "department": "교직팀",
    "time_points": "2.0/2.0",
    "personeel": "0",
    "remaining_seats": "40",
    "schedule_room": "금 16:00-16:50 (진리관 11407-조호제)\n금 17:00-17:50 (진리관 11407-조호제)",
    "target": "순수외국인입학생 제한"
  },
]
```

## find-connected-major (연계전공)

- 연계전공 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-connected-major --year <YEAR> --semester <SEMESTER> --major-name <MAJOR_NAME>
```

### Examples

```bash
rusaint-cli find-connected-major --year 2025 --semester 1 --major-name "융합창업연계" 
```

```json
[
  {
    "syllabus": null,
    "category": "교선",
    "sub_category": "연계2-융합창업",
    "abeek_info": null,
    "field": "[‘23이후]자기개발·진로탐색\n['20,'21~'22]공동체/리더십,숭실품성-인성과리더십\n['19]숭실품성-인성과리더십\n['16-'18]숭실품성(리더십-리더십이론및실천)\n['15이전]인성과리더쉽(핵심-창의)",
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "실전SNS마케팅과실무",
    "division": null,
    "professor": "오현석",
    "department": "창업교육·지원팀",
    "time_points": "3.0/3.0",
    "personeel": "0",
    "remaining_seats": "50",
    "schedule_room": "금 15:00-16:15 (조만식기념관 12406-오현석)\n금 16:30-17:45 (조만식기념관 12406-오현석)",
    "target": "전체"
  },
]
```

## find-united-major (융합전공)

- 융합전공 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-united-major --year <YEAR> --semester <SEMESTER> --major-name <MAJOR_NAME>
```

### Examples

```bash
rusaint-cli find-united-major --year 2025 --semester 1 --major-name "빅데이터융합"
```

```json
[
  {
    "syllabus": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.HAS_PLAN_PAPER_SALV_WD_CE.1",
    "category": "전필-소프트",
    "sub_category": "복선-소프트/융선-빅데이터융합",
    "abeek_info": "공학주제-소프트공인증/인필-소프트공인증",
    "field": null,
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "데이터베이스",
    "division": null,
    "professor": "이상호",
    "department": "소프트웨어학부",
    "time_points": "3.0/3.0",
    "personeel": "0",
    "remaining_seats": "50",
    "schedule_room": "월 수 09:00-10:15 (정보과학관 21303-이상호)",
    "target": "3학년 소프트"
  },
]
```

## find-recognized-other-major (타전공인정과목)

- 타전공인정과목 정보를 가져와 json 파일로 추출합니다.

### Usage
```bash
rusaint-cli find-recognized-other-major --year <YEAR> --semester <SEMESTER> --college <COLLEGE> --department <DEPARTMENT> --major <MAJOR>
```

### Examples
- 세부 전공이 있는 경우
```bash
rusaint-cli find-recognized-other-major --year 2025 --semester 1 --college "공과대학" --department "건축학부" --major "건축공학전공"
```
- 세부 전공이 없는 경우
```bash
rusaint-cli find-recognized-other-major --year 2025 --semester 1 --college "IT대학" --department "컴퓨터학부"
```

```json
[
  {
    "syllabus": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.HAS_PLAN_PAPER_SALV_WD_CE.1",
    "category": "전선-컴퓨터",
    "sub_category": null,
    "abeek_info": null,
    "field": null,
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "디지털공학",
    "division": null,
    "professor": "차형태",
    "department": "전자정보공학부 전자공학전공",
    "time_points": "3.0/3.0",
    "personeel": "0",
    "remaining_seats": "40",
    "schedule_room": "화 13:30-14:45 (형남공학관 05104 (김낙경강의실)-차형태)\n목 13:30-14:45 (형남공학관 05104 (김낙경강의실)-차형태)",
    "target": "2학년 전자공학전공(IT융합 수강제한)"
  },
]
```

## find-cyber (숭실사이버대)

- 숭실사이버대 과목 정보를 가져와 json 파일로 추출합니다.

### Usage

```bash
rusaint-cli find-cyber --year <YEAR> --semester <SEMESTER>
```

### Examples

```bash
rusaint-cli find-cyber --year 2025 --semester 1
```

```json
[
  {
    "syllabus": null,
    "category": "교선",
    "sub_category": null,
    "abeek_info": null,
    "field": "숭실사이버대과목\n[‘23이후]자기개발·진로탐색\n['20,'21~'22]공동체/리더십,숭실품성-인성과리더십\n['19]숭실품성-인성과리더십\n['16-'18]균형교양(사회과학-문화및문명)\n['15이전]생활과건강(실용-생활)",
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "식품과건강(숭실사이버대)",
    "division": null,
    "professor": "",
    "department": "학사팀",
    "time_points": "3.0/3.0",
    "personeel": "0",
    "remaining_seats": "200",
    "schedule_room": "",
    "target": "전체"
  },
]
```