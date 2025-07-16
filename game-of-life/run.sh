#!/bin/bash

# 콘웨이의 생명게임 - 실행 스크립트
echo "🧬 콘웨이의 생명게임을 시작합니다..."

# 현재 디렉토리 확인
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 오류: game-of-life 디렉토리에서 실행해주세요"
    exit 1
fi

# wasm-pack 확인 및 설치
if ! command -v wasm-pack &> /dev/null; then
    echo "🔧 wasm-pack을 설치하는 중..."
    cargo install wasm-pack
fi

# 빌드
echo "🔨 프로젝트를 빌드하는 중..."
wasm-pack build --target web --out-dir pkg

# 빌드 결과 확인
if [ $? -eq 0 ]; then
    echo "✅ 빌드 완료!"
    
    # www 디렉토리로 파일 복사
    echo "📁 파일을 www 디렉토리로 복사하는 중..."
    mkdir -p www/pkg
    cp -r pkg/* www/pkg/
    
    # 서버 실행
    echo "🚀 웹 서버를 시작하는 중..."
    echo "🌐 브라우저에서 http://localhost:8000을 열어주세요"
    echo "⏹️  서버를 중지하려면 Ctrl+C를 누르세요"
    echo ""
    
    cd www
    python3 -m http.server 8000
else
    echo "❌ 빌드 실패!"
    exit 1
fi 