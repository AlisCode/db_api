use crate::mounter::Mounter;

/// The Repository trait
/// Gives a Repository the ability to return all of its routes.
pub trait Repository {
    /// Mounts all the routes of the Repository
    /// Using the mounter
    fn mount<M: Mounter>(mounter: &M, url: &str);
}
