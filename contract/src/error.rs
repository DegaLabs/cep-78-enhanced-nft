use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum NFTCoreError {
    InvalidAccount = 1,
    MissingInstaller = 2,
    InvalidInstaller = 3,
    UnexpectedKeyVariant = 4,
    MissingTokenOwner = 5,
    InvalidTokenOwner = 6,
    FailedToGetArgBytes = 7,
    FailedToCreateDictionary = 8,
    MissingStorageUref = 9,
    InvalidStorageUref = 10,
    MissingOwnersUref = 11,
    InvalidOwnersUref = 12,
    FailedToAccessStorageDictionary = 13,
    FailedToAccessOwnershipDictionary = 14,
    DuplicateMinted = 15,
    FailedToConvertToCLValue = 16,
    MissingCollectionName = 17,
    InvalidCollectionName = 18,
    FailedToSerializeMetaData = 19,
    MissingAccount = 20,
    MissingMintingStatus = 21,
    InvalidMintingStatus = 22,
    MissingCollectionSymbol = 23,
    InvalidCollectionSymbol = 24,
    MissingTotalTokenSupply = 25,
    InvalidTotalTokenSupply = 26,
    MissingTokenID = 27,
    InvalidTokenID = 28,
    MissingTokenOwners = 29,
    MissingAccountHash = 30,
    InvalidAccountHash = 31,
    TokenSupplyDepleted = 32,
    MissingOwnedTokensDictionary = 33,
    TokenAlreadyBelongsToMinterFatal = 34,
    FatalTokenIdDuplication = 35,
    InvalidMinter = 36,
    MissingPublicMinting = 37,
    InvalidPublicMinting = 38,
    MissingInstallerKey = 39,
    FailedToConvertToAccountHash = 40,
    InvalidBurner = 41,
    PreviouslyBurntToken = 42,
    MissingAllowMinting = 43,
    InvalidAllowMinting = 44,
    MissingNumberOfMintedTokens = 45,
    InvalidNumberOfMintedTokens = 46,
    MissingTokenMetaData = 47,
    InvalidTokenMetaData = 48,
    MissingApprovedAccountHash = 49,
    InvalidApprovedAccountHash = 50,
    MissingApprovedTokensDictionary = 51,
    TokenAlreadyApproved = 52,
    MissingApproveAll = 53,
    InvalidApproveAll = 54,
    MissingOperator = 55,
    InvalidOperator = 56,
    Phantom = 57,
    ContractAlreadyInitialized = 58,
    MintingIsPaused = 59,
    FailureToParseAccountHash = 60,
    VacantValueInDictionary = 61,
    MissingOwnershipMode = 62,
    InvalidOwnershipMode = 63,
    InvalidTokenMinter = 64,
    MissingOwnedTokens = 65,
    InvalidAccountKeyInDictionary = 66,
}

impl From<NFTCoreError> for ApiError {
    fn from(e: NFTCoreError) -> Self {
        ApiError::User(e as u16)
    }
}
