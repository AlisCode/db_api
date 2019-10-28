#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use db_api::database::ConnectionRetriever;
use db_api::endpoint::GenericEndpoint;
use db_api::mounter::rocket::RocketMounter;
use db_api::mounter::Mounter;
use db_api::retriever::IndexedParamRetriever;
use db_api::retriever::{BodyRetriever, DeserializeRetriever, UniqueStateRetriever};
use db_api::Method;
use rocket::Rocket;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
#[doc = r" The request guard type."]
struct ExampleDb(
    pub  ::rocket_contrib::databases::r2d2::PooledConnection<
        <diesel::SqliteConnection as ::rocket_contrib::databases::Poolable>::Manager,
    >,
);
#[doc = r" The pool type."]
struct ExampleDbPool(
    ::rocket_contrib::databases::r2d2::Pool<
        <diesel::SqliteConnection as ::rocket_contrib::databases::Poolable>::Manager,
    >,
);
impl ExampleDb {
    #[doc = r" Returns a fairing that initializes the associated database"]
    #[doc = r" connection pool."]
    pub fn fairing() -> impl ::rocket::fairing::Fairing {
        use ::rocket_contrib::databases::Poolable;
        ::rocket::fairing::AdHoc::on_attach("\'rocket_example_sqlite\' Database Pool", |rocket| {
            let pool = ::rocket_contrib::databases::database_config(
                "rocket_example_sqlite",
                rocket.config(),
            )
            .map(<diesel::SqliteConnection>::pool);
            match pool {
                Ok(Ok(p)) => Ok(rocket.manage(ExampleDbPool(p))),
                Err(config_error) => {
                    ::rocket::logger::error(&::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Database configuration failure: \'", "\'"],
                        &match (&"rocket_example_sqlite",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    )));
                    ::rocket::logger::error_(&::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &match (&config_error,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ),
                    ));
                    Err(rocket)
                }
                Ok(Err(pool_error)) => {
                    ::rocket::logger::error(&::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Failed to initialize pool for \'", "\'"],
                        &match (&"rocket_example_sqlite",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    )));
                    ::rocket::logger::error_(&::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &match (&pool_error,) {
                                (arg0,) => {
                                    [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                                }
                            },
                        ),
                    ));
                    Err(rocket)
                }
            }
        })
    }
    #[doc = r" Retrieves a connection of type `Self` from the `rocket`"]
    #[doc = r" instance. Returns `Some` as long as `Self::fairing()` has been"]
    #[doc = r" attached and there is at least one connection in the pool."]
    pub fn get_one(rocket: &::rocket::Rocket) -> Option<Self> {
        rocket
            .state::<ExampleDbPool>()
            .and_then(|pool| pool.0.get().ok())
            .map(ExampleDb)
    }
}
impl ::std::ops::Deref for ExampleDb {
    type Target = diesel::SqliteConnection;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::ops::DerefMut for ExampleDb {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<'a, 'r> ::rocket::request::FromRequest<'a, 'r> for ExampleDb {
    type Error = ();
    fn from_request(
        request: &'a ::rocket::request::Request<'r>,
    ) -> ::rocket::request::Outcome<Self, ()> {
        use ::rocket::{http::Status, Outcome};
        let pool = request.guard::<::rocket::State<ExampleDbPool>>()?;
        match pool.0.get() {
            Ok(conn) => Outcome::Success(ExampleDb(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
impl ConnectionRetriever<diesel::sqlite::Sqlite> for ExampleDb {
    type Output = diesel::SqliteConnection;
    fn retrieve_connection(self) -> Self::Output {
        {
            ::std::rt::begin_panic(
                "not yet implemented",
                &("rocket_example/src/main.rs", 21u32, 9u32),
            )
        }
    }
}
fn handle(_unit: ()) -> String {
    "Handled!".to_owned()
}
fn retrievers() {}
struct A {
    pub val: u32,
    other: u32,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_A: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for A {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "A",
                false as usize + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "val",
                &self.val,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "other",
                &self.other,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_A: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for A {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 2",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "val" => _serde::export::Ok(__Field::__field0),
                        "other" => _serde::export::Ok(__Field::__field1),
                        _ => _serde::export::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"val" => _serde::export::Ok(__Field::__field0),
                        b"other" => _serde::export::Ok(__Field::__field1),
                        _ => _serde::export::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<A>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = A;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "struct A")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct A with 2 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct A with 2 elements",
                                ));
                            }
                        };
                    _serde::export::Ok(A {
                        val: __field0,
                        other: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<u32> = _serde::export::None;
                    let mut __field1: _serde::export::Option<u32> = _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("val"),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::export::Option::is_some(&__field1) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("other"),
                                    );
                                }
                                __field1 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => match _serde::private::de::missing_field("val") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => match _serde::private::de::missing_field("other") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    _serde::export::Ok(A {
                        val: __field0,
                        other: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["val", "other"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "A",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<A>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for A {
    #[inline]
    fn clone(&self) -> A {
        match *self {
            A {
                val: ref __self_0_0,
                other: ref __self_0_1,
            } => A {
                val: ::core::clone::Clone::clone(&(*__self_0_0)),
                other: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
fn handle_a(a: A) -> String {
    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
        &["val is "],
        &match (&a.val,) {
            (arg0,) => [::core::fmt::ArgumentV1::new(
                arg0,
                ::core::fmt::Display::fmt,
            )],
        },
    ))
}
fn retrievers_a() -> DeserializeRetriever<A> {
    DeserializeRetriever::new()
}
fn handle_str(my_str: String) -> String {
    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
        &["Handled : "],
        &match (&my_str,) {
            (arg0,) => [::core::fmt::ArgumentV1::new(
                arg0,
                ::core::fmt::Display::fmt,
            )],
        },
    ))
}
fn retrievers_str() -> BodyRetriever<String> {
    BodyRetriever::new()
}
struct Counter {
    val: Arc<Mutex<u32>>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Counter {
    #[inline]
    fn clone(&self) -> Counter {
        match *self {
            Counter {
                val: ref __self_0_0,
            } => Counter {
                val: ::core::clone::Clone::clone(&(*__self_0_0)),
            },
        }
    }
}
impl Counter {
    pub fn new() -> Self {
        Counter {
            val: Arc::new(Mutex::new(0)),
        }
    }
    pub fn count(&self) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val += 1;
        *val
    }
    pub fn add_val(&self, v: u32) -> u32 {
        let mut val = self.val.lock().unwrap();
        *val += v;
        *val
    }
}
fn handle_count(counter: Arc<Counter>) -> String {
    let new_val = counter.count();
    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
        &["This page has been visited ", " times"],
        &match (&new_val,) {
            (arg0,) => [::core::fmt::ArgumentV1::new(
                arg0,
                ::core::fmt::Display::fmt,
            )],
        },
    ))
}
fn retrievers_count() -> UniqueStateRetriever<Counter> {
    UniqueStateRetriever::new()
}
fn handle_count_deser((counter, a): (Arc<Counter>, A)) -> String {
    let new_val = counter.add_val(a.val);
    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
        &["This common count is "],
        &match (&new_val,) {
            (arg0,) => [::core::fmt::ArgumentV1::new(
                arg0,
                ::core::fmt::Display::fmt,
            )],
        },
    ))
}
fn retrievers_count_deser() -> (UniqueStateRetriever<Counter>, DeserializeRetriever<A>) {
    (UniqueStateRetriever::new(), DeserializeRetriever::new())
}
fn handle_url_param(a: u32) -> String {
    ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
        &["The value is "],
        &match (&a,) {
            (arg0,) => [::core::fmt::ArgumentV1::new(
                arg0,
                ::core::fmt::Display::fmt,
            )],
        },
    ))
}
fn retrievers_url_param() -> IndexedParamRetriever<u32> {
    IndexedParamRetriever::new(1)
}
fn main() {
    let endpoint_test = GenericEndpoint::new("/test".into(), Method::GET, handle, retrievers);
    let endpoint_str =
        GenericEndpoint::new("/test_str".into(), Method::GET, handle_str, retrievers_str);
    let endpoint_deser_a =
        GenericEndpoint::new("/deser".into(), Method::POST, handle_a, retrievers_a);
    let endpoint_counter =
        GenericEndpoint::new("/count".into(), Method::GET, handle_count, retrievers_count);
    let endpoint_counter_deser = GenericEndpoint::new(
        "/count_deser".into(),
        Method::POST,
        handle_count_deser,
        retrievers_count_deser,
    );
    let endpoint_url_param = GenericEndpoint::new(
        "/param/<id>".into(),
        Method::GET,
        handle_url_param,
        retrievers_url_param,
    );
    let rocket = Rocket::ignite().manage(Arc::new(Counter::new()));
    let mut mounter = RocketMounter::new(rocket);
    mounter.mount_service(endpoint_test.rocket());
    mounter.mount_service(endpoint_str.rocket());
    mounter.mount_service(endpoint_deser_a.rocket());
    mounter.mount_service(endpoint_counter.rocket());
    mounter.mount_service(endpoint_counter_deser.rocket());
    mounter.mount_service(endpoint_url_param.rocket());
    let rocket = mounter.finish();
    rocket.launch();
}
