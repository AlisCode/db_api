use crate::mounter::Mounter;

/// The Repository trait
/// Gives a Repository the ability to mount all of its routes
/// to a mounter
pub trait Repository {
    /// Mounts all the routes of the Repository
    /// Using the mounter
    fn mount<M: Mounter + ?Sized>(mounter: &mut M);
}
