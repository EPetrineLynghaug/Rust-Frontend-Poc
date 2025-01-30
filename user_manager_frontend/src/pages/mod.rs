// Declare a module named `home`. The code for this module
// is located in a file called "home.rs" in the same directory.
mod home;
// Re-export (make available to parent modules) the `HomePage`
// entity defined inside the `home` module.
pub use home::HomePage;

// Declare a module named `login`. The code for this module
// is located in a file called "login.rs".
mod login;
// Re-export the `LoginPage` from the `login` module so it
// can be used by other parts of the application.
pub use login::LoginPage;

// Declare a module named `article` in the file "article.rs".
mod article;
// Re-export the `ArticlePage` from the `article` module.
pub use article::ArticlePage;
