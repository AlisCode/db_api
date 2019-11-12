use crate::mounter::Mounter;

pub mod rest_template;

pub trait Template<Marker> {
    fn mount_template(mounter: impl Mounter);
}
