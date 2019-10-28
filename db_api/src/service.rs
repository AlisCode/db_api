use crate::mounter::Mounter;

pub trait Service<M>
where
   M: Mounter + ?Sized,
{
   /// Transforms the endpoint into a specific Route
   /// Calls implementation-specific functions to mount
   /// said route into the final application
   fn mount_service(self, mounter: &mut M);
}
