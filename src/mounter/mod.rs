use crate::repository::Repository;
use crate::retriever::RetrieverBackend;
use crate::service::Service;

/// A Mounter is a struct capable of mouting `Service`s on an instance
/// of a Web App Framework, e.g. Rocket's `Rocket` or actix-web's `App`
pub trait Mounter
where
    Self::Back: RetrieverBackend,
{
    type Back;
    fn mount_service<S: Service<Self>>(&mut self, service: S)
    where
        Self: Sized;
    fn mount_repository<Repo: Repository>(&mut self, _rep: &Repo) {
        Repo::mount::<Self>(self);
    }
}

#[cfg(feature = "rocket_integ")]
pub mod rocket;

#[cfg(feature = "rocket_integ")]
pub mod rocket_other;
