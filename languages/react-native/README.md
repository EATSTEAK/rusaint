<h1 align="center">@rusaint/react-native</h1>
<p align="center" style="font-style: italic;">빠르고 간편하며 믿을 수 있는 숭실대학교 u-saint 클라이언트</p>
<p align="center">
    <a href="https://github.com/EATSTEAK/rusaint"><img alt="GitHub Badge" src="https://img.shields.io/badge/github-eatsteak/rusaint-8da0cb?style=for-the-badge&labelColor=555555&logo=github"></a>
    <a href="https://www.npmjs.com/package/@rusaint/react-native"><img alt="NPM Distribution" src="https://img.shields.io/npm/v/%40rusaint%2Freact-native?style=for-the-badge&logo=npm&color=CB3837"></a>
    <a href="https://docs.rs/rusaint"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rusaint-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs"></a>
   <a href="https://github.com/EATSTEAK/rusaint/LICENSE.md"><img alt="License" src="https://img.shields.io/github/license/EATSTEAK/rusaint?style=for-the-badge"></a>
</p>

---

rusaint(_ru-saint, 루세인트_)는 [숭실대학교 u-saint](https://saint.ssu.ac.kr)를 정확하고 빠르게, 간편하게 파싱하고 다양한 환경에서 조작할 수 있는 Rust 기반 비공식
u-saint 클라이언트입니다.

u-saint의 기반인 [SAP Web Dynpro](https://en.wikipedia.org/wiki/Web_Dynpro)에서 사용하는 Lightspeed 라이브러리의 최소 동작을 구현하여 안전하게
u-saint 내부 요소들을 조작하고 파싱할 수 있습니다.

- **JS 런타임 없음** — JS 런타임 없이 자체적으로 요청과 응답에 따른 처리를 수행하므로 HTTPS 요청이 가능한 모든 환경에서 실행 가능합니다.
- **빠른 속도** — 네이티브 환경으로 컴파일되는 Rust를 이용하고, 휴리스틱 없이 요청이 완료되면 곧바로 실행되어 빠르게 u-saint 를 조작 및 파싱 가능합니다.
- **멀티플랫폼 지원** — UniFFI를 통한 Kotlin, Swift, Python(예정) 지원 및 Node.js 용 WASM Wrapper(예정)를 제공하여 다양한 플랫폼에서 간편하게 이용할 수 있습니다.
- **간편한 기능 정의** — rusaint 에서 지원하지 않는 u-saint 애플리케이션에 대한 파싱 및 지원을 제공하는 API를 이용해 간편하게 정의할 수 있습니다.

## 설치

```bash
pnpm add @rusaint/react-native # or yarn, npm
```

## 문서

[docs.rs](https://docs.rs/rusaint)

## 예시

```typescript
import {
  CourseScheduleApplicationBuilder,
  Lecture,
  LectureCategory,
  LectureCategoryBuilder,
  SemesterType,
  USaintSessionBuilder,
  type CourseScheduleApplicationInterface,
} from '@rusaint/react-native';
import { useRef, useState, useEffect } from 'react';

// Creates an anonymous session
const session = new USaintSessionBuilder().anonymous();

// A hook for find lectures from the CourseScheduleApplication
export const useFindLectures = (
  year: number,
  semester: SemesterType,
  category: LectureCategory
) => {
  const clientRef = useRef<CourseScheduleApplicationInterface | null>(null);
  const [result, setResult] = useState<Lecture[]>([]);
  useEffect(() => {
    (async () => {
      const client = await new CourseScheduleApplicationBuilder().build(
        session
      );
      clientRef.current = client;
    })();
  }, []);

  useEffect(() => {
    (async () => {
      let result = await clientRef.current?.findLectures(
        year,
        semester,
        category
      );
      setResult(result || []);
      console.log('Lectures fetched:', result);
    })();
  }, [year, semester, category]);
  return result;
};

// Creates a category outside of the component
const category = new LectureCategoryBuilder().major(
  'IT대학',
  '글로벌미디어학부',
  undefined
);

// Use in the component
const result = useFindLectures(2025, SemesterType.One, category);
```

## Expo에서 사용하기

현재 `@rusaint/react-native`는 Expo Module을 지원하지 않습니다. 따라서 Expo 프로젝트에서 사용하기 위해서는 `@react-native-community/cli`의 autolink 기능을 활성화하여 Turbo Module을 autolink 해야 합니다.

```bash
pnpm add @react-native-community/cli -D # or use yarn, npm
```

으로 `@react-native-community/cli`를 설치합니다.

```properties
# .env
EXPO_USE_COMMUNITY_AUTOLINKING=1 # Enable autolinking by @react-native-community/cli
```

`EXPO_USE_COMMUNITY_AUTOLINKING` 환경변수를 `expo prebuild` 과정에 제공하여 모듈의 autolink를 활성화합니다.

> [!WARNING]
> Community autolinking을 활성화 하면 Expo Go를 사용할 수 없습니다.
