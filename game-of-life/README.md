# 🧬 Conway's Game of Life

콘웨이의 생명게임을 Rust와 WebAssembly로 구현한 발표용 프로젝트입니다.

## 📋 기능

- ✅ **게임판 크기 조정**: 32x32부터 128x128까지 동적 크기 조정
- ✅ **제어 기능**: 시작/정지/초기화 버튼
- ✅ **사전 정의된 패턴**: 글라이더, 펄사, 비콘, 토드, 우주선 등 5가지 패턴
- ✅ **실시간 성능 모니터링**: FPS 카운터 및 통계 표시
- ✅ **에러 로깅**: 콘솔 에러 로깅 시스템
- ✅ **인터랙티브**: 클릭으로 개별 셀 토글 가능
- ✅ **랜덤 생성**: 랜덤 초기 상태 생성 기능

## 🚀 빌드 및 실행 방법

### 1. 전제 조건

- Rust (최신 stable 버전)
- wasm-pack (WebAssembly 빌드 도구)
- Python 3 (로컬 서버용)

### 2. 빌드

```bash
# 프로젝트 디렉토리로 이동
cd game-of-life

# 빌드 스크립트 실행
./build.sh
```

### 3. 실행

```bash
# www 디렉토리로 이동
cd www

# 로컬 서버 시작
python3 -m http.server 8000
```

브라우저에서 `http://localhost:8000`에 접속하면 게임을 시연할 수 있습니다.

## 🎮 사용 방법

### 기본 조작
- **Play/Pause**: 게임 시작/정지
- **Clear**: 모든 셀 초기화
- **Random**: 랜덤 초기 상태 생성
- **Size Slider**: 게임판 크기 조정

### 패턴 로드
- 드롭다운 메뉴에서 사전 정의된 패턴 선택
- **Glider**: 우하향으로 이동하는 패턴
- **Pulsar**: 주기적으로 깜빡이는 패턴
- **Beacon**: 2x2 블록이 교대로 나타나는 패턴
- **Toad**: 수직/수평으로 변하는 패턴
- **Spaceship**: 우측으로 이동하는 우주선 패턴

### 실시간 모니터링
- **Generation**: 현재 세대 수
- **Live Cells**: 살아있는 셀 개수
- **Size**: 현재 게임판 크기
- **FPS**: 현재 프레임 속도

## 🔧 기술 스택

- **Backend**: Rust + WebAssembly
- **Frontend**: HTML5 Canvas + JavaScript
- **Build Tool**: wasm-pack
- **Performance**: wee_alloc 메모리 최적화

## 📊 성능 최적화

- **메모리 효율성**: Vec<Cell> 사용으로 메모리 사용량 최소화
- **성능 모니터링**: 브라우저 콘솔에서 실시간 성능 추적
- **에러 처리**: panic hook으로 런타임 오류 로깅

## 🎯 발표 시연 포인트

1. **웹 어셈블리의 성능**: 큰 게임판에서도 부드러운 애니메이션
2. **Rust의 안정성**: 메모리 안전성과 타입 안전성
3. **실시간 상호작용**: 클릭으로 즉시 반응하는 인터페이스
4. **다양한 패턴**: 생명게임의 흥미로운 수학적 패턴들
5. **성능 모니터링**: 실시간 FPS 및 통계 표시

## 📁 프로젝트 구조

```
game-of-life/
├── src/
│   ├── lib.rs          # 메인 WebAssembly 모듈
│   └── utils.rs        # 유틸리티 함수
├── www/
│   ├── index.html      # 웹 인터페이스
│   └── pkg/           # 빌드된 WebAssembly 파일들
├── Cargo.toml         # Rust 의존성 설정
├── build.sh           # 빌드 스크립트
└── README.md          # 프로젝트 설명
```

## 🐛 트러블슈팅

만약 빌드 중 오류가 발생하면:

1. Rust 버전 확인: `rustc --version`
2. wasm-pack 설치: `cargo install wasm-pack`
3. 권한 오류 시: `chmod +x build.sh`

## 📚 Conway's Game of Life 규칙

1. **죽음 (Underpopulation)**: 살아있는 셀이 2개 미만의 이웃을 가지면 죽음
2. **생존 (Survival)**: 살아있는 셀이 2-3개의 이웃을 가지면 생존
3. **죽음 (Overpopulation)**: 살아있는 셀이 3개 초과의 이웃을 가지면 죽음
4. **탄생 (Reproduction)**: 죽은 셀이 정확히 3개의 이웃을 가지면 탄생

---

🎯 **발표 완료 후 질문 대응 준비**: 웹 어셈블리 성능, Rust 메모리 안전성, 실시간 렌더링 최적화 등에 대한 기술적 질문에 답변할 수 있습니다. 