use std::fmt::Display;

use bdk_wallet::WalletPersister;

pub enum Persister {
    #[cfg(feature = "sqlite")]
    Connection(bdk_wallet::rusqlite::Connection),
    #[cfg(feature = "redb")]
    RedbStore(bdk_redb::Store),
}

#[derive(Debug, thiserror::Error)]
pub enum PersisterError {
    #[cfg(feature = "sqlite")]
    SqliteError(bdk_wallet::rusqlite::Error),
    #[cfg(feature = "redb")]
    RedbError(bdk_redb::error::BdkRedbError),
}

#[cfg(feature = "sqlite")]
impl From<bdk_wallet::rusqlite::Error> for PersisterError {
    fn from(value: bdk_wallet::rusqlite::Error) -> Self {
        PersisterError::SqliteError(value)
    }
}

#[cfg(feature = "redb")]
impl From<bdk_redb::error::BdkRedbError> for PersisterError {
    fn from(value: bdk_redb::error::BdkRedbError) -> Self {
        PersisterError::RedbError(value)
    }
}

impl Display for PersisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "sqlite")]
            PersisterError::SqliteError(err) => err.fmt(f),
            #[cfg(feature = "redb")]
            PersisterError::RedbError(err) => err.fmt(f),
        }
    }
}

impl WalletPersister for Persister {
    type Error = PersisterError;

    fn initialize(persister: &mut Self) -> Result<bdk_wallet::ChangeSet, Self::Error> {
        match persister {
            #[cfg(feature = "sqlite")]
            Persister::Connection(connection) => {
                WalletPersister::initialize(connection).map_err(PersisterError::from)
            }
            #[cfg(feature = "redb")]
            Persister::RedbStore(store) => {
                WalletPersister::initialize(store).map_err(PersisterError::from)
            }
        }
    }

    fn persist(persister: &mut Self, changeset: &bdk_wallet::ChangeSet) -> Result<(), Self::Error> {
        match persister {
            #[cfg(feature = "sqlite")]
            Persister::Connection(connection) => {
                WalletPersister::persist(connection, changeset).map_err(PersisterError::from)
            }
            #[cfg(feature = "redb")]
            Persister::RedbStore(store) => {
                WalletPersister::persist(store, changeset).map_err(PersisterError::from)
            }
        }
    }
}
