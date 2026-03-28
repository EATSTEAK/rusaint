<h1 align="center">rusaint-cli</h1>
<p align="center" style="font-style: italic;">CLI 환경에서 숭실대학교 u-saint 학사 정보를 조회하는 커맨드라인 도구</p>
<p align="center">
    <a href="https://crates.io/crates/rusaint-cli"><img alt="crates.io" src="https://img.shields.io/crates/v/rusaint-cli.svg?style=for-the-badge&color=fc8d62&logo=rust"></a>
    <a href="https://github.com/EATSTEAK/rusaint"><img alt="GitHub Badge" src="https://img.shields.io/badge/github-eatsteak/rusaint-8da0cb?style=for-the-badge&labelColor=555555&logo=github"></a>
   <a href="https://github.com/EATSTEAK/rusaint/LICENSE.md"><img alt="License" src="https://img.shields.io/github/license/EATSTEAK/rusaint?style=for-the-badge"></a>
</p>

---

`rusaint-cli`는 [rusaint](https://github.com/EATSTEAK/rusaint) 기반의 커맨드라인 도구로, 숭실대학교 u-saint의 다양한 학사 정보를 터미널에서 바로 조회할 수 있습니다.

- **강의시간표 조회** — 과목명, 전공, 교양, 채플, 교직 등 다양한 기준으로 강의 검색
- **성적 조회** — 학기별 성적, 이수구분별 성적, 전체 요약 등
- **학생정보 조회** — 일반정보, 졸업, 자격증, 은행계좌 등
- **수강신청 내역** — 학기별 수강신청 과목 조회
- **졸업사정표** — 졸업요건 및 학생정보 확인
- **강의평가 검색** — 강의명, 교수명, 과목코드로 강의평가 조회
- **장학금 조회** — 장학금 수혜 내역 확인
- **개인시간표** — 학기별 개인 시간표 조회
- **JSON 출력** — `--format json` 옵션으로 JSON 형태 출력 지원

## 설치

Rust가 설치되어 있어야 합니다. [rustup](https://rustup.rs)으로 설치할 수 있습니다.

```bash
# Rust 설치 (미설치 시)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# rusaint-cli 설치
cargo install rusaint-cli
```

> **Note:** 설치 후 실행 바이너리 이름은 `rusaint`입니다.

## 빠른 시작

### 환경 변수 설정

u-saint에 로그인하려면 SSO 인증 정보가 필요합니다. 실행 위치에 `.env` 파일을 생성합니다.

```env
SSO_ID={학번}           # 예: 20203123
SSO_PASSWORD={비밀번호} # 예: 1q2w1q2w!
```

또는 `--env-file` 옵션으로 `.env` 파일 경로를 직접 지정할 수 있습니다.

### 세션 파일 사용 (선택)

매번 로그인하는 대신, 세션을 파일로 저장하여 재사용할 수 있습니다.

```bash
rusaint create-session -o session.json
# 이후 --session-file 옵션으로 세션 파일 사용
rusaint --session-file session.json student-info general
```

### 글로벌 옵션

| 옵션 | 설명 | 기본값 |
|---|---|---|
| `--session-file <PATH>` | 세션 JSON 파일 경로 | SSO_ID/SSO_PASSWORD 사용 |
| `--env-file <PATH>` | .env 파일 경로 | 현재 디렉토리의 .env |
| `--format <FORMAT>` | 출력 포맷 (`human` 또는 `json`) | `human` |
| `-o, --output <PATH>` | 결과 출력 파일 경로 | stdout |

### 값 타입

**학기 (`-s`, `--semester`)**: `1` (1학기), `summer` (여름학기), `2` (2학기), `winter` (겨울학기)

**과정구분 (`-t`, `--course-type`)**: `bachelor` (학사, 기본값), `master` (석사), `phd` (박사), `phd-integrated` (석박사통합), `research` (연구)

## 명령어

### create-session — 세션 파일 생성

u-saint 로그인 세션을 JSON 파일로 저장합니다. 저장된 세션 파일은 `--session-file` 옵션으로 재사용할 수 있습니다.

```bash
rusaint create-session -o session.json
```

### course-schedule — 강의시간표 조회

강의시간표 분류에 대응하는 다양한 서브커맨드를 제공합니다.

| 서브커맨드 | 설명 |
|---|---|
| `by-lecture` | 강의명으로 검색 |
| `major` | 학부전공별 검색 |
| `required-elective` | 교양필수 검색 |
| `optional-elective` | 교양선택 검색 |
| `chapel` | 채플 강의 검색 |
| `education` | 교직 검색 |
| `connected-major` | 연계전공 검색 |
| `united-major` | 융합전공 검색 |
| `recognized-other-major` | 타전공인정 검색 |
| `cyber` | 숭실사이버대학교 검색 |
| `graduated` | 대학원 검색 |
| `find-by-professor` | 교수명으로 검색 |

#### 공통 옵션

| 옵션 | 설명 |
|---|---|
| `-y, --year <YEAR>` | 학년도 |
| `-s, --semester <SEMESTER>` | 학기 |
| `--detailed` | 상세 정보 포함 |
| `--fetch-syllabus` | 강의계획서 포함 (`--detailed` 자동 포함) |

#### by-lecture (과목명 검색)

```bash
rusaint course-schedule by-lecture -y 2025 -s 1 -k "대학글쓰기"
```

#### major (학부전공)

```bash
# 세부 전공이 있는 경우
rusaint course-schedule major -y 2025 -s 1 -c "공과대학" -d "건축학부" -m "건축공학전공"

# 세부 전공이 없는 경우
rusaint course-schedule major -y 2025 -s 1 -c "IT대학" -d "컴퓨터학부"
```

#### required-elective (교양필수)

```bash
rusaint course-schedule required-elective -y 2025 -s 1 -n "대학한국어1"
```

#### optional-elective (교양선택)

```bash
rusaint course-schedule optional-elective -y 2025 -s 1 -n "['23이후]과학·기술"
```

#### chapel (채플)

```bash
rusaint course-schedule chapel -y 2025 -s 1 -n "비전채플"
```

#### education (교직)

```bash
rusaint course-schedule education -y 2025 -s 1
```

#### connected-major (연계전공)

```bash
rusaint course-schedule connected-major -y 2025 -s 1 -n "융합창업연계"
```

#### united-major (융합전공)

```bash
rusaint course-schedule united-major -y 2025 -s 1 -n "빅데이터융합"
```

#### recognized-other-major (타전공인정)

```bash
rusaint course-schedule recognized-other-major -y 2025 -s 1 -c "IT대학" -d "컴퓨터학부"
```

#### cyber (숭실사이버대)

```bash
rusaint course-schedule cyber -y 2025 -s 1
```

#### graduated (대학원)

```bash
rusaint course-schedule graduated -y 2025 -s 1 -c "일반대학원" -d "컴퓨터학부"
```

#### find-by-professor (교수명 검색)

```bash
rusaint course-schedule find-by-professor -y 2025 -s 1 -k "김지학"
```

### student-info — 학생정보 조회

| 서브커맨드 | 설명 |
|---|---|
| `general` | 일반 학생정보 |
| `graduation` | 졸업정보 |
| `qualifications` | 자격증 정보 |
| `work` | 직장정보 |
| `family` | 가족정보 |
| `religion` | 종교정보 |
| `transfer` | 편입정보 |
| `bank-account` | 은행계좌 정보 |
| `academic-record` | 학적상태 이력 |
| `research-bank-account` | 연구비 계좌 |

```bash
rusaint student-info general
rusaint --format json student-info graduation
```

### grades — 성적 조회

| 서브커맨드 | 설명 |
|---|---|
| `recorded-summary` | 전체 학기 성적 요약 (기록부 기준) |
| `certificated-summary` | 전체 학기 성적 요약 (증명서 기준) |
| `by-classification` | 이수구분별 성적 조회 |
| `semesters` | 학기별 성적 목록 |
| `classes` | 과목별 성적 목록 |
| `class-detail` | 개별 과목 성적 상세 |

```bash
# 전체 성적 요약
rusaint grades recorded-summary

# 특정 학기 과목별 성적
rusaint grades classes -y 2025 -s 1

# 상세 정보 포함
rusaint grades classes -y 2025 -s 1 --include-details

# 개별 과목 성적 상세
rusaint grades class-detail -y 2025 -s 1 -c "HIS01001"

# 석사 과정 성적
rusaint grades recorded-summary -t master
```

### chapel-info — 채플 정보 조회

```bash
rusaint chapel-info information -y 2025 -s 1
```

### registration — 수강신청 내역 조회

```bash
rusaint registration lectures -y 2025 -s 1
```

### graduation — 졸업사정표 조회

| 서브커맨드 | 설명 |
|---|---|
| `student-info` | 졸업사정 학생정보 |
| `requirements` | 졸업요건 |

```bash
rusaint graduation student-info
rusaint graduation requirements
```

### assessment — 강의평가 검색

```bash
# 강의명으로 검색
rusaint assessment find -y 2025 -s 1 --lecture-name "데이터베이스"

# 교수명으로 검색
rusaint assessment find -y 2025 -s 1 --professor-name "이상호"

# 과목코드로 검색
rusaint assessment find -y 2025 -s 1 --lecture-code 12345
```

### personal-schedule — 개인시간표 조회

```bash
rusaint personal-schedule schedule -y 2025 -s 1
```

### scholarships — 장학금 조회

```bash
rusaint scholarships list
```

## JSON 출력 예시

`--format json` 옵션을 사용하면 JSON 형태로 결과를 출력합니다.

```bash
rusaint --format json course-schedule by-lecture -y 2025 -s 1 -k "대학글쓰기"
```

```json
[
  {
    "category": "교필",
    "code": "SALV_WD_TABLE.ID_DE0D9128A4327646C94670E2A892C99C:VIEW_TABLE.SE_SHORT_SALV_WD_CE.1",
    "name": "(외국인을위한)대학글쓰기",
    "professor": "김지학",
    "department": "교양교육운영팀",
    "time_points": "2.0/2.0",
    "remaining_seats": "30",
    "schedule_room": "월 11:00-11:50 (전산관 19330-김지학)\n월 12:00-12:50 (전산관 19330-김지학)",
    "target": "전체학년 전체;순수외국인입학생 (대상외수강제한)"
  }
]
```

파일로 저장하려면 `-o` 옵션을 사용합니다.

```bash
rusaint --format json -o grades.json grades classes -y 2025 -s 1
```
