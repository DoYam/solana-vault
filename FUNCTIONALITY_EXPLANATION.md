# Solana Vault - 기능 및 동작 방식 설명

## 📋 프로젝트 개요

Solana Vault는 Solana 블록체인에서 안전하게 자산(SOL)을 보관하고 관리할 수 있는 스마트 컨트랙트 프로그램입니다.

## 🎯 주요 기능

### 1. **Vault 초기화 (Initialize)**
- **목적**: 사용자가 자신만의 Vault 계정을 생성
- **동작 방식**:
  1. 사용자가 `initialize()` 함수를 호출
  2. PDA(Program Derived Address)를 사용해 고유한 Vault 주소 생성
     - Seeds: `["vault", owner의 공개키]`
     - 같은 owner는 항상 같은 Vault 주소를 가짐
  3. Vault 계정에 다음 정보 저장:
     - `owner`: Vault 소유자의 공개키
     - `total_deposited`: 총 입금액 (초기값: 0)
     - `total_withdrawn`: 총 출금액 (초기값: 0)
  4. 계정 생성 비용(rent)은 owner가 지불

### 2. **입금 (Deposit)**
- **목적**: 누구나 Vault에 SOL을 입금 가능
- **동작 방식**:
  1. 사용자가 `deposit(amount)` 함수 호출
  2. Solana 시스템 프로그램의 `transfer` 명령어 사용
  3. 사용자의 지갑에서 Vault 계정으로 SOL 전송
  4. Vault의 `total_deposited` 값 증가 (오버플로우 방지)
  5. 트랜잭션 로그에 입금 내역 기록

**특징**:
- 인증 불필요 (누구나 입금 가능)
- 안전한 전송 (시스템 프로그램 사용)
- 통계 추적 (총 입금액 기록)

### 3. **출금 (Withdraw)**
- **목적**: Vault 소유자만 자금 출금 가능
- **동작 방식**:
  1. 소유자가 `withdraw(amount, recipient)` 함수 호출
  2. **보안 검증 1**: 소유자 확인
     - Vault의 `owner`와 트랜잭션 서명자가 일치하는지 확인
     - 일치하지 않으면 `Unauthorized` 오류 발생
  3. **보안 검증 2**: 잔액 확인
     - Vault 계정의 잔액이 출금액 이상인지 확인
     - 부족하면 `InsufficientFunds` 오류 발생
  4. Vault 계정에서 recipient 계정으로 SOL 직접 전송
  5. Vault의 `total_withdrawn` 값 증가 (오버플로우 방지)
  6. 트랜잭션 로그에 출금 내역 기록

**특징**:
- 소유자만 출금 가능 (보안)
- 잔액 검증 (안전성)
- 효율적인 전송 (직접 lamports 조작)

## 🔐 보안 기능

### 1. **PDA (Program Derived Address)**
- Vault 주소가 결정론적으로 생성됨
- 같은 owner는 항상 같은 Vault 주소를 가짐
- 주소 충돌 방지

### 2. **소유자 인증**
- 출금 시 소유자 확인 필수
- 다른 사람이 출금 시도 시 거부

### 3. **오버플로우 방지**
- `checked_add()` 사용으로 정수 오버플로우 방지
- 안전한 수학 연산

### 4. **잔액 검증**
- 출금 전 Vault 잔액 확인
- 부족한 자금 출금 시도 방지

## 📊 데이터 구조

```rust
pub struct Vault {
    pub owner: Pubkey,           // 소유자 공개키 (32 bytes)
    pub total_deposited: u64,    // 총 입금액 (8 bytes)
    pub total_withdrawn: u64,    // 총 출금액 (8 bytes)
}
```

**계정 크기**: 8 (discriminator) + 32 + 8 + 8 = 56 bytes

## 🔄 실행 순서 예시

### 시나리오: Alice가 Vault를 만들고 Bob이 입금, Alice가 출금

1. **초기화**:
   ```
   Alice → initialize()
   → Vault 생성 (owner: Alice)
   → total_deposited: 0, total_withdrawn: 0
   ```

2. **입금**:
   ```
   Bob → deposit(1 SOL)
   → Bob의 지갑에서 Vault로 1 SOL 전송
   → total_deposited: 1 SOL
   ```

3. **출금**:
   ```
   Alice → withdraw(0.5 SOL, recipient: Charlie)
   → 소유자 확인 ✓
   → 잔액 확인 ✓ (1 SOL >= 0.5 SOL)
   → Vault에서 Charlie로 0.5 SOL 전송
   → total_withdrawn: 0.5 SOL
   ```

4. **잘못된 출금 시도**:
   ```
   Bob → withdraw(0.3 SOL, recipient: Bob)
   → 소유자 확인 ✗ (Bob != Alice)
   → Unauthorized 오류 발생
   ```

## 🧪 테스트 시나리오

1. **Vault 초기화 테스트**
   - Vault가 올바르게 생성되는지 확인
   - owner, total_deposited, total_withdrawn 초기값 확인

2. **입금 테스트**
   - SOL 입금 성공 확인
   - total_deposited 증가 확인
   - Vault 잔액 증가 확인

3. **출금 테스트**
   - 소유자가 출금 성공 확인
   - total_withdrawn 증가 확인
   - recipient 잔액 증가 확인

4. **보안 테스트 1: 무단 출금 방지**
   - 다른 사용자가 출금 시도
   - Unauthorized 오류 발생 확인

5. **보안 테스트 2: 잔액 부족 방지**
   - 잔액보다 많은 금액 출금 시도
   - InsufficientFunds 오류 발생 확인

## 💡 핵심 개념

### PDA (Program Derived Address)
- 프로그램이 결정론적으로 생성하는 주소
- Seeds를 사용해 고유한 주소 생성
- 예: `[b"vault", owner.key()]` → 항상 같은 주소

### Account Constraints
- Anchor가 자동으로 검증하는 계정 조건
- `init`: 새 계정 생성
- `mut`: 계정 수정 가능
- `seeds`: PDA 검증
- `signer`: 서명자 확인

### Lamports
- Solana의 최소 단위 (1 SOL = 1,000,000,000 lamports)
- 모든 금액은 lamports 단위로 처리

