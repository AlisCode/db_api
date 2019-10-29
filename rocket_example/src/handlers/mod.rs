pub mod body;
pub mod counter;
pub mod deser;
pub mod hero_count;
pub mod multiple_retrievers;
pub mod url_param;
pub mod void;

pub use body::endpoint_body;
pub use counter::endpoint_counter;
pub use deser::endpoint_deser;
pub use hero_count::endpoint_count_heroes;
pub use multiple_retrievers::endpoint_multiple_retrievers;
pub use url_param::endpoint_url_param;
pub use void::endpoint_void;
