use crate::mounter::Mounter;

pub trait Service<M> where M: Mounter + ?Sized {
   fn mount_service(self, mounter: &mut M);
}
