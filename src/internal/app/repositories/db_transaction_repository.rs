use sqlx::{Error, PgPool, Postgres, Transaction};

// Define the DbTransactionRepository trait
pub trait DbTransactionRepository: Send + Sync {
    async fn begin_transaction(&self) -> Result<Transaction<Postgres>, Error>;
    async fn commit_transaction(&self, transaction: Transaction<'_, Postgres>) -> Result<(), Error>;
}

// Implementation of the DbTransactionRepository
#[derive(Clone, Debug)]
pub struct DbTransactionRepositoryImpl {
    pool: PgPool,
}

impl DbTransactionRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        DbTransactionRepositoryImpl { pool }
    }
}

impl DbTransactionRepository for DbTransactionRepositoryImpl {
    async fn begin_transaction(&self) -> Result<Transaction<Postgres>, Error> {
        match self.pool.begin().await {
            Ok(transaction) => Ok(transaction),
            Err(error) => Err(error)
        }
    }

    async fn commit_transaction(&self, mut transaction: Transaction<'_, Postgres>) -> Result<(), Error> {
        match transaction.commit().await {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }
    }
}
