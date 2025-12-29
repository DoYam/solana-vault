# Solana Vault

Solana 블록체인에서 안전한 자산 관리를 위한 Vault 프로그램입니다.

## 프로젝트 개요

이 프로젝트는 Solana + Anchor 프레임워크를 사용하여 개발된 Vault 프로그램입니다.

## 개발 계획

### Phase 1: 개발 (4시간)
- [x] Solana + Anchor 설치
- [ ] Vault 프로그램 작성
- [ ] 테스트 작성
- [ ] 로컬 테스트 통과

### Phase 2: 배포 (1시간)
- [ ] Devnet 배포
- [ ] Program ID 확인
- [ ] Solana Explorer에서 확인
- [ ] 테스트 재실행

### Phase 3: 문서화 (30분)
- [x] README 작성
- [ ] 스크린샷 추가
- [x] GitHub 업로드

## 기술 스택

- **Solana CLI**: 1.18.20
- **Anchor**: 0.32.1
- **Rust**: 1.92.0

## 시작하기

### 사전 요구사항

- Rust (1.92.0 이상)
- Solana CLI
- Anchor CLI
- Node.js (v20.18.0 이상 권장)

### 설치

```bash
# 의존성 설치
npm install

# 또는
yarn install
```

### 빌드

```bash
anchor build
```

### 테스트

```bash
anchor test
```

## 프로젝트 구조

```
solana-anchor-project/
├── Anchor.toml          # Anchor 설정 파일
├── programs/            # Solana 프로그램
│   └── solana-vault/
│       └── src/
│           └── lib.rs   # 메인 프로그램 코드
├── tests/               # 테스트 파일
│   └── solana-vault.ts
└── migrations/          # 배포 스크립트
```

## 라이선스

ISC

