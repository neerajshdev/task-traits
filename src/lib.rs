
pub mod task { 

    use async_trait::async_trait;
    use std::error::Error;
    
    /// Trait for synchronous tasks.
    /// This trait should be implemented by tasks that do not require asynchronous operations.
    pub trait SyncTask: Send + Sync {
        /// The output type of the task.
        type Output;
        /// The error type returned by the task.
        type Error: Error + Send + Sync + 'static;
    
        /// Executes the synchronous task and returns the result.
        fn execute(&self) -> Result<Self::Output, Self::Error>;
    }
    
    /// Trait for asynchronous tasks.
    /// This trait should be implemented by tasks that perform asynchronous operations.
    #[async_trait]
    pub trait AsyncTask: Send + Sync {
        /// The output type of the task.
        type Output;
        /// The error type returned by the task.
        type Error: Error + Send + Sync + 'static;
    
        /// Executes the asynchronous task and returns the result.
        async fn execute(&self) -> Result<Self::Output, Self::Error>;
    } 
}