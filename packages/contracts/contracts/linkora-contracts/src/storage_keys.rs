use soroban_sdk::{contracttype, symbol_short, Address, BytesN, String, Symbol};

/// Persistent and temporary contract keys. Variants document the storage shape
/// at each call site and keep serialization stable during refactors.
#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    Post(u64),
    Profile(Address),
    Following(Address),
    Followers(Address),
    Pool(Symbol),
    Like(u64, Address),
    AuthorPosts(Address),
    Blocks(Address),
    BlockedBy(Address),
    UsernameIndex(String),
    TipCooldown(u64, Address),
    PoolDepositCooldown(Symbol, Address),
    Edge(Address, Address),
    FollowingCount(Address),
    FollowersCount(Address),
    FollowingIdx(Address, u32),
    FollowersIdx(Address, u32),
    FollowingPos(Address, Address),
    FollowersPos(Address, Address),
    GraphMigrated(Address),
    DmPublicKey(Address),
    CredentialRoot(Address),
    NullifierSet(Address, BytesN<32>),
    CredentialAuthority,
    GovProposal(u64),
    GovVote(u64, Address),
    GovConfig,
    GovProposalCount,
    GovOpenProposalCount(Address),
    OracleKey(Symbol),
    AttestationNullifier(BytesN<32>),
    Report(u64, Address),
    ReportCount(u64),
    OpenReports(Address),
    DeletedPost(u64),
    DeletedProfile(Address),
    PostLikersCount(u64),
    PostLikersIdx(u64, u32),
    PostReportersIdx(u64, u32),
    PostTipCooldownsCount(u64),
    PostTipCooldownsIdx(u64, u32),
    UpgradeProposal,
    ReputationScore(Address),
    ReputationDecayLedger(Address),
    Banned(Address),
    RewardEpoch,
    RewardClaimable(Address, u64),
    RewardClaimed(Address, u64),
    RewardToken(u64),
}

// Short instance-storage keys. Keep every literal in this module so storage
// names cannot drift between contract methods.
pub const POST_CT: Symbol = symbol_short!("POST_CT");
pub const PROFILE_CREATED_CT: Symbol = symbol_short!("PROF_CT");
pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const TREASURY: Symbol = symbol_short!("TREASURY");
pub const FEE_BPS: Symbol = symbol_short!("FEE_BPS");
pub const INITIALIZED: Symbol = symbol_short!("INIT");
pub const TIP_COOLDOWN_WINDOW: Symbol = symbol_short!("TIP_CD_W");
pub const REGISTERED_USERS: Symbol = symbol_short!("R_USERS");
pub const RENT_RATE_BPS_KEY: Symbol = symbol_short!("RENT_BPS");
pub const MODERATION_SLASH_BPS: Symbol = symbol_short!("MOD_SL_B");
pub const CONTRACT_STATE: Symbol = symbol_short!("CT_STATE");
pub const ROLES: Symbol = symbol_short!("ROLES");
pub const PAUSED: Symbol = symbol_short!("PAUSED");
pub const MAX_POST_LEN_KEY: Symbol = symbol_short!("MAX_POST");
pub const MAX_BIO_LEN_KEY: Symbol = symbol_short!("MAX_BIO");
