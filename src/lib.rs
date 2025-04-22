
pub mod task { 
    use async_trait::async_trait;
    
    /// Trait for synchronous tasks.
    /// This trait should be implemented by tasks that do not require asynchronous operations.
    pub trait SyncTask: Send + Sync {
        /// The output type of the task
        type Output;
    
        /// Executes the synchronous task and returns the result.
        fn execute(&self) -> Self::Output;
    }
    
    /// Trait for asynchronous tasks.
    /// This trait should be implemented by tasks that perform asynchronous operations.
    #[async_trait]
    pub trait AsyncTask: Send + Sync {
        /// The output type of the task.
        type Output;    
        /// Executes the asynchronous task and returns the result.
        async fn execute(&self) -> Self::Output;
    } 
}