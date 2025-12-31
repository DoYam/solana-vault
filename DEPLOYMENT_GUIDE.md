# Devnet 배포 가이드

## 현재 상태

- ✅ Devnet 설정 완료
- ✅ 키페어 생성 완료 (Pubkey: EcaJPb1NDVLHK7tNJJwJXqxyiNQ9X8FmHMwMJErTRAAP)
- ✅ Devnet에서 2 SOL 에어드롭 완료
- ✅ Anchor.toml에 devnet 설정 추가 완료
- ⚠️ Solana 플랫폼 도구 설치 필요

## 배포 단계

### 1. Solana 플랫폼 도구 설치

터미널에서 다음 명령어를 실행하세요:

```bash
# Solana 설치 스크립트 실행
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# PATH에 추가
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# 플랫폼 도구 설치
solana-install init 1.18.20
```

또는 Homebrew를 사용한 경우:

```bash
# Solana 플랫폼 도구 설치
brew install solana-platform-tools
```

### 2. 프로젝트 디렉토리로 이동

```bash
cd /Users/kdy/Desktop/astar/solana-anchor-project
```

### 3. Devnet 설정 확인

```bash
# Devnet으로 설정되어 있는지 확인
solana config get

# 출력 예시:
# RPC URL: https://api.devnet.solana.com
```

### 4. 잔액 확인

```bash
solana balance

# 2 SOL 이상이 있어야 배포 가능
```

부족하면:

```bash
solana airdrop 2
```

### 5. 프로그램 빌드

```bash
# Cargo 환경 변수 로드
source $HOME/.cargo/env

# 빌드 실행
anchor build
```

빌드가 성공하면:
- `target/deploy/solana_vault.so` 파일 생성
- `target/idl/solana_vault.json` 생성
- `target/types/solana_vault.ts` 생성

### 6. Devnet 배포

```bash
anchor deploy
```

배포가 성공하면 Program ID가 출력됩니다.

### 7. Program ID 확인 및 업데이트

배포 후 출력된 Program ID를 확인하고:

```bash
# Program ID 확인
solana address -k target/deploy/solana_vault-keypair.json
```

`Anchor.toml` 파일을 업데이트:

```toml
[programs.devnet]
solana_vault = "실제_Program_ID"
```

그리고 `programs/solana-vault/src/lib.rs` 파일도 업데이트:

```rust
declare_id!("실제_Program_ID");
```

### 8. Solana Explorer에서 확인

배포된 Program ID를 Solana Explorer에서 확인:

```
https://explorer.solana.com/address/프로그램_ID?cluster=devnet
```

### 9. 테스트 재실행

```bash
anchor test --skip-local-validator
```

## 문제 해결

### build-sbf 명령어를 찾을 수 없는 경우

```bash
# Solana 플랫폼 도구 확인
which cargo-build-sbf

# 없으면 설치
cargo install --git https://github.com/solana-labs/solana cargo-build-sbf
```

### 네트워크 오류

SSL 오류가 발생하면:

```bash
# 인증서 업데이트
brew update
brew upgrade ca-certificates
```

### 잔액 부족

```bash
# Devnet에서 SOL 요청
solana airdrop 2

# 여러 번 시도 가능
```

## 배포 후 확인 사항

- [ ] Program ID 확인
- [ ] Solana Explorer에서 프로그램 확인
- [ ] Anchor.toml에 Program ID 업데이트
- [ ] lib.rs에 Program ID 업데이트
- [ ] 테스트 재실행 성공

## 참고

- Devnet은 테스트 네트워크이므로 실제 가치가 없습니다
- Devnet SOL은 무료로 받을 수 있습니다
- 배포 비용은 약 2-3 SOL 정도 소요됩니다

