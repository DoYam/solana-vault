# Solana Vault - 프로젝트 구조 및 코드 동작 방식 상세 설명

## 📁 프로젝트 구조

```
solana-anchor-project/
├── Anchor.toml              # Anchor 프레임워크 설정 파일
├── Cargo.toml               # Rust 워크스페이스 설정
├── package.json             # Node.js 의존성 및 스크립트
├── rust-toolchain.toml      # Rust 버전 설정
│
├── programs/                # Solana 프로그램 (스마트 컨트랙트)
│   └── solana-vault/
│       ├── Cargo.toml       # 프로그램별 Rust 설정
│       └── src/
│           └── lib.rs       # 메인 프로그램 코드 ⭐
│
├── tests/                   # 테스트 파일
│   └── solana-vault.ts      # TypeScript 테스트 코드
│
├── migrations/              # 배포 스크립트
│   └── deploy.ts           # 프로그램 배포 스크립트
│
└── target/                  # 빌드 산출물 (자동 생성)
    ├── deploy/             # 배포 가능한 프로그램 바이너리
    ├── idl/                # IDL (Interface Definition Language)
    └── types/              # TypeScript 타입 정의
```

## 🔧 각 파일의 역할

### 1. Anchor.toml - 프로젝트 설정

```toml
[programs.localnet]
solana_vault = "11111111111111111111111111111111"

[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

**역할:**
- 프로그램 ID 정의 (빌드 시 자동 생성됨)
- 네트워크 설정 (localnet/devnet/mainnet)
- 지갑 경로 지정
- 테스트 스크립트 설정

### 2. programs/solana-vault/src/lib.rs - 핵심 프로그램 코드

이 파일이 **Solana 블록체인에 배포되는 스마트 컨트랙트**입니다.

#### 구조 분석:

**A. 프로그램 모듈 선언**
```rust
#[program]
pub mod solana_vault {
    // 함수들...
}
```
- `#[program]`: Anchor가 이 모듈을 Solana 프로그램으로 인식
- 모든 공개 함수가 블록체인에서 호출 가능한 "instruction"이 됨

**B. 데이터 구조 (Vault Account)**
```rust
#[account]
pub struct Vault {
    pub owner: Pubkey,           // 32 bytes
    pub total_deposited: u64,   // 8 bytes
    pub total_withdrawn: u64,   // 8 bytes
}
```
- `#[account]`: Anchor가 자동으로 직렬화/역직렬화 처리
- 총 크기: 8(discriminator) + 32 + 8 + 8 = 56 bytes
- 이 구조가 블록체인 계정에 저장됨

**C. Account Constraints (계정 제약 조건)**

각 함수마다 필요한 계정을 정의:

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,                                    // 새 계정 생성
        payer = owner,                           // owner가 비용 지불
        space = 8 + 32 + 8 + 8,                 // 계정 크기
        seeds = [b"vault", owner.key().as_ref()], // PDA seeds
        bump                                     // Bump seed
    )]
    pub vault: Account<'info, Vault>,
    // ...
}
```

**Anchor의 마법:**
- `init`: 계정이 없으면 자동 생성
- `seeds`: PDA 주소 자동 계산 및 검증
- `payer`: 계정 생성 비용 자동 처리
- `mut`: 계정 수정 가능 표시

## 🔄 코드 동작 흐름

### 시나리오 1: Vault 초기화

```
1. 클라이언트 (TypeScript)
   ↓
   program.methods.initialize()
   .accounts({ vault, owner, systemProgram })
   .rpc()
   
2. Anchor 프레임워크
   ↓
   - Account constraints 검증
   - PDA 주소 계산 (seeds: ["vault", owner])
   - 트랜잭션 구성
   
3. Solana 네트워크
   ↓
   - 트랜잭션 전송
   - 프로그램 실행
   
4. 프로그램 (lib.rs)
   ↓
   pub fn initialize(ctx: Context<Initialize>) {
       vault.owner = owner.key();      // 소유자 저장
       vault.total_deposited = 0;      // 초기화
       vault.total_withdrawn = 0;      // 초기화
   }
   
5. 블록체인
   ↓
   - Vault 계정 생성 (56 bytes)
   - 데이터 저장
   - 트랜잭션 완료
```

### 시나리오 2: 입금 (Deposit)

```
1. 사용자가 deposit(amount) 호출
   ↓
2. Anchor가 계정 검증
   - vault 계정이 존재하는지
   - PDA 주소가 맞는지
   - user가 서명했는지
   ↓
3. 프로그램 실행
   pub fn deposit(ctx: Context<Deposit>, amount: u64) {
       // 시스템 프로그램 호출
       invoke(
           &system_instruction::transfer(
               &user.key(),      // 보내는 사람
               &vault.key(),     // 받는 사람
               amount            // 금액
           ),
           &[user, vault, system_program]
       )?;
       
       // 통계 업데이트
       vault.total_deposited += amount;
   }
   ↓
4. 결과
   - user 지갑: SOL 감소
   - vault 계정: SOL 증가
   - vault.total_deposited: 증가
```

### 시나리오 3: 출금 (Withdraw)

```
1. 소유자가 withdraw(amount, recipient) 호출
   ↓
2. Anchor가 계정 검증
   - vault 계정 존재 확인
   - owner가 서명했는지 확인
   ↓
3. 프로그램 실행
   pub fn withdraw(ctx: Context<Withdraw>, amount: u64) {
       // 보안 검증 1: 소유자 확인
       require!(
           vault.owner == owner.key(),
           ErrorCode::Unauthorized
       );
       
       // 보안 검증 2: 잔액 확인
       require!(
           vault.lamports >= amount,
           ErrorCode::InsufficientFunds
       );
       
       // 직접 lamports 전송 (효율적)
       vault.lamports -= amount;
       recipient.lamports += amount;
       
       // 통계 업데이트
       vault.total_withdrawn += amount;
   }
   ↓
4. 결과
   - vault 계정: SOL 감소
   - recipient 계정: SOL 증가
   - vault.total_withdrawn: 증가
```

## 🎯 핵심 개념

### 1. PDA (Program Derived Address)

```rust
seeds = [b"vault", owner.key().as_ref()]
```

**동작 원리:**
1. Seeds를 해시 함수에 입력
2. 프로그램 ID와 결합
3. 유효한 주소가 나올 때까지 bump 값 조정
4. 결정론적 주소 생성 (같은 입력 → 같은 주소)

**장점:**
- 소유자가 없어도 계정 생성 가능
- 예측 가능한 주소
- 프로그램이 계정을 소유

### 2. Account Constraints

Anchor가 자동으로 검증하는 것들:

```rust
#[account(
    init,           // 계정이 없으면 생성
    payer = owner,  // owner가 비용 지불
    space = 56,     // 계정 크기
    seeds = [...],  // PDA 검증
    bump            // Bump seed 자동 찾기
)]
```

**검증 내용:**
- 계정 존재 여부
- 계정 크기
- PDA 주소 정확성
- 서명자 확인
- 권한 확인

### 3. Context와 Accounts

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;  // 계정 접근
    let owner = &ctx.accounts.owner;      // 소유자 접근
    // ...
}
```

**Context 구조:**
- `ctx.accounts`: 함수에 필요한 모든 계정
- `ctx.program_id`: 프로그램 ID
- `ctx.remaining_accounts`: 추가 계정들

### 4. 에러 처리

```rust
#[error_code]
pub enum ErrorCode {
    Unauthorized,
    InsufficientFunds,
    Overflow,
}
```

**사용:**
```rust
require!(condition, ErrorCode::Unauthorized);
// 조건이 false면 에러 반환
```

## 🧪 테스트 코드 동작

### 테스트 구조:

```typescript
describe("solana-vault", () => {
  // 1. 설정
  const program = anchor.workspace.solanaVault;
  const owner = provider.wallet;
  
  // 2. PDA 계산
  [vaultPda, vaultBump] = await PublicKey.findProgramAddress(
    [Buffer.from("vault"), owner.publicKey.toBuffer()],
    program.programId
  );
  
  // 3. 테스트 실행
  it("Initializes vault", async () => {
    await program.methods.initialize()
      .accounts({ vault: vaultPda, ... })
      .rpc();
      
    // 4. 검증
    const vault = await program.account.vault.fetch(vaultPda);
    expect(vault.owner).to.equal(owner.publicKey);
  });
});
```

**테스트 흐름:**
1. 로컬 validator 시작 (자동)
2. 프로그램 배포 (자동)
3. 테스트 실행
4. 계정 상태 검증
5. 정리

## 🔐 보안 메커니즘

### 1. 소유자 인증
```rust
require!(vault.owner == owner.key(), ErrorCode::Unauthorized);
```
- Vault의 owner와 트랜잭션 서명자 비교
- 일치하지 않으면 거부

### 2. 잔액 검증
```rust
require!(vault.lamports >= amount, ErrorCode::InsufficientFunds);
```
- 출금 전 잔액 확인
- 부족하면 거부

### 3. 오버플로우 방지
```rust
vault.total_deposited = vault.total_deposited
    .checked_add(amount)
    .ok_or(ErrorCode::Overflow)?;
```
- `checked_add()`: 오버플로우 시 None 반환
- 안전한 수학 연산

### 4. PDA 검증
- Anchor가 자동으로 seeds 검증
- 잘못된 주소 접근 방지

## 📊 데이터 흐름

### 입금 시:
```
User 지갑 (100 SOL)
    ↓ transfer
Vault 계정 (0 SOL)
    ↓
Vault 계정 (100 SOL)
Vault.total_deposited = 100
```

### 출금 시:
```
Vault 계정 (100 SOL)
    ↓ 직접 전송
Recipient 지갑 (0 SOL)
    ↓
Vault 계정 (50 SOL)
Recipient 지갑 (50 SOL)
Vault.total_withdrawn = 50
```

## 🛠️ 빌드 및 배포 과정

### 1. 빌드
```bash
anchor build
```
**과정:**
- Rust 코드 컴파일
- Solana BPF 바이너리 생성
- IDL 생성 (TypeScript 타입)
- Program ID 생성

### 2. 테스트
```bash
anchor test
```
**과정:**
- 로컬 validator 시작
- 프로그램 배포
- 테스트 실행
- 결과 검증

### 3. 배포
```bash
anchor deploy
```
**과정:**
- Devnet/Mainnet에 배포
- Program ID 확인
- 트랜잭션 서명

## 💡 핵심 포인트 요약

1. **Anchor 프레임워크**: 복잡한 Solana 개발을 간소화
2. **PDA**: 프로그램이 소유하는 결정론적 주소
3. **Account Constraints**: 자동 계정 검증 및 생성
4. **Context**: 함수에 필요한 모든 정보 제공
5. **보안**: 다층 검증 (소유자, 잔액, 오버플로우)

이 구조로 안전하고 효율적인 Solana 스마트 컨트랙트를 구축할 수 있습니다!

