use clap:: {Args, Parser, Subcommand};
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show users
    User(UserCommand),
    /// Create, update, delete or show videos
    Video(VideoCommand),
    // /// Create or show views
    // VIew(ViewCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    command: UserSubcommand,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    command: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),
    /// Update an existing user
    Update(UpdateUser),
    /// Delete a user
    Delete(DeleteEntity),
    /// Show all users
    Show,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create,
    /// Update an existing video
    Update,
    /// Delete a video
    Delete,
    /// Show all videos
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    name: String,
    /// The email address of the user
    email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The name of the user
    name: String,
    /// The email address of the user
    email: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The name of the user
    name: String,
}
