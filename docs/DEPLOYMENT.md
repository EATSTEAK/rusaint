# 배포 설정 가이드

이 문서는 rusaint를 fork하여 자체 배포를 설정하려는 사용자를 위한 가이드입니다.

## 개요

릴리스 워크플로우는 `main` 브랜치에 `Cargo.toml` 변경이 push되면 자동으로 트리거됩니다. 버전이 변경된 경우 다음 플랫폼에 배포됩니다:

| 플랫폼       | 배포 대상          | 필요 시크릿               |
| ------------ | ------------------ | ------------------------- |
| Rust         | crates.io          | Trusted Publisher 설정    |
| Android      | Maven Central      | Maven Central + GPG       |
| iOS          | rusaint-ios 저장소 | Apple 인증서 + GitHub PAT |
| Python       | PyPI               | PyPI API 토큰             |
| React Native | npm                | Apple 인증서 + npm 토큰   |

---

## 1. Rust (crates.io)

### Action Items

1. [crates.io](https://crates.io)에 로그인
2. Account Settings → Trusted Publishers 이동
3. 새 Publisher 추가:
   - **Repository owner**: `{YOUR_GITHUB_USERNAME}`
   - **Repository name**: `rusaint`
   - **Workflow filename**: `release.yml`
   - **Environment**: _(비워둠)_

> 📌 `rust-lang/crates-io-auth-action`이 자동으로 토큰을 발급받습니다.

---

## 2. Android (Maven Central)

### 필요 시크릿

| 시크릿 이름             | 설명                         |
| ----------------------- | ---------------------------- |
| `MAVENCENTRAL_USERNAME` | Sonatype Central 사용자 이름 |
| `MAVENCENTRAL_PASSWORD` | Sonatype Central 비밀번호    |
| `GPG_PASSPHRASE`        | GPG 키 패스프레이즈          |
| `GPG_PUBLIC_KEY`        | GPG 공개 키 (armor 형식)     |
| `GPG_SECRET_KEY`        | GPG 비밀 키 (armor 형식)     |

### Action Items

1. **Sonatype Central 계정 생성**

   - [central.sonatype.com](https://central.sonatype.com)에서 계정 생성
   - namespace(`dev.eatsteak` → `dev.{your-namespace}`) 소유권 확인

2. **GPG 키 생성**

   ```bash
   # 키 생성
   gpg --full-generate-key

   # 공개 키 내보내기
   gpg --armor --export YOUR_KEY_ID

   # 비밀 키 내보내기
   gpg --armor --export-secret-keys YOUR_KEY_ID

   # 키 서버에 업로드
   gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID
   ```

3. **GitHub 시크릿 등록**

   - Repository → Settings → Secrets and variables → Actions
   - 위 5개 시크릿 추가

4. **패키지 네임스페이스 변경**
   - `languages/kotlin/lib/build.gradle.kts`에서 `dev.eatsteak` → 본인 namespace로 변경

---

## 3. iOS (rusaint-ios 저장소)

### 필요 시크릿

| 시크릿 이름                    | 설명                                     |
| ------------------------------ | ---------------------------------------- |
| `IOS_BUILD_CERTIFICATE_BASE64` | Apple 인증서 (.p12) Base64 인코딩        |
| `IOS_P12_PASSWORD`             | .p12 파일 비밀번호                       |
| `IOS_KEYCHAIN_PASSWORD`        | 임시 키체인 비밀번호 (임의 값)           |
| `IOS_GITHUB_TOKEN`             | rusaint-ios 저장소 write 권한이 있는 PAT |

### Action Items

1. **Apple Developer 계정에서 인증서 생성**

   - [developer.apple.com](https://developer.apple.com) → Certificates, Identifiers & Profiles
   - "Apple Development" 인증서 생성 및 다운로드

2. **인증서를 .p12로 내보내기** (macOS)

   ```bash
   # Keychain Access에서 인증서 + 개인 키 선택 → 내보내기 (.p12)

   # Base64 인코딩
   base64 -i certificate.p12 | pbcopy
   ```

3. **rusaint-ios 저장소 생성**

   - 새 저장소 생성: `{YOUR_USERNAME}/rusaint-ios`
   - Swift Package 구조 설정 필요

4. **GitHub PAT 생성**

   - Settings → Developer settings → Personal access tokens
   - `repo` 권한으로 Fine-grained token 생성 (rusaint-ios 저장소 대상)

5. **GitHub 시크릿 등록**

6. **워크플로우 수정**
   - `.github/workflows/ios-release.yml`에서 `EATSTEAK/rusaint-ios` → `{YOUR_USERNAME}/rusaint-ios`로 변경

---

## 4. Python (PyPI)

### 필요 시크릿

| 시크릿 이름      | 설명          |
| ---------------- | ------------- |
| `PYPI_API_TOKEN` | PyPI API 토큰 |

### Action Items

1. **PyPI 계정 생성**

   - [pypi.org](https://pypi.org)에서 계정 생성

2. **API 토큰 발급**

   - Account settings → API tokens → Add API token
   - Scope: 전체 계정 또는 특정 프로젝트

3. **GitHub 시크릿 등록**

   - `PYPI_API_TOKEN`에 `pypi-`로 시작하는 토큰 값 입력

4. **패키지 이름 변경** (선택)
   - `languages/python/pyproject.toml`에서 패키지 이름 변경

---

## 5. React Native (npm)

### 필요 시크릿

| 시크릿 이름                    | 설명              |
| ------------------------------ | ----------------- |
| `IOS_BUILD_CERTIFICATE_BASE64` | (iOS와 동일)      |
| `IOS_P12_PASSWORD`             | (iOS와 동일)      |
| `IOS_KEYCHAIN_PASSWORD`        | (iOS와 동일)      |
| `NPM_TOKEN`                    | npm 퍼블리시 토큰 |

### Action Items

1. **npm 계정 생성**

   - [npmjs.com](https://www.npmjs.com)에서 계정 생성

2. **npm 조직 생성** (scoped package용)

   - `@rusaint` → `@{your-scope}`로 변경 시 새 조직 필요

3. **Access Token 발급**

   - Profile → Access Tokens → Generate New Token
   - Type: **Automation** (CI/CD용)

4. **GitHub 시크릿 등록**

5. **패키지 scope 변경**
   - `languages/react-native/package.json`에서 `@rusaint` → `@{your-scope}`로 변경

---

## 시크릿 요약 체크리스트

```
Repository → Settings → Secrets and variables → Actions
```

### Repository Secrets

- [ ] `MAVENCENTRAL_USERNAME`
- [ ] `MAVENCENTRAL_PASSWORD`
- [ ] `GPG_PASSPHRASE`
- [ ] `GPG_PUBLIC_KEY`
- [ ] `GPG_SECRET_KEY`
- [ ] `IOS_BUILD_CERTIFICATE_BASE64`
- [ ] `IOS_P12_PASSWORD`
- [ ] `IOS_KEYCHAIN_PASSWORD`
- [ ] `IOS_GITHUB_TOKEN`
- [ ] `PYPI_API_TOKEN`
- [ ] `NPM_TOKEN`

### Repository Variables (선택)

- [ ] `IOS_DEPLOYMENT_TARGET` - iOS 최소 버전 (기본값: `14.0`)

---

## 배포 트리거

### 자동 배포

- `main` 브랜치에 `Cargo.toml` push 시 버전이 변경되었으면 자동 배포

### 수동 배포

- Actions → Release → Run workflow

### 개별 플랫폼 배포

각 플랫폼별 워크플로우를 수동으로 실행 가능:

- Android Release
- iOS Release
- Python Release
- React Native Release
