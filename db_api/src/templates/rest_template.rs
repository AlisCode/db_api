use crate::database::ConnectionRetriever;
use crate::mounter::Mounter;
use crate::responders::JsonResponse;
use crate::retriever::{DbRetriever, IndexedParamRetriever};
use crate::templates::Template;

use diesel::associations::HasTable;
use diesel::connection::Connection;
use diesel::dsl::{Find, Select};
use diesel::expression::count::CountStar;
use diesel::query_builder::{AsQuery, Query, QueryFragment, QueryId};
use diesel::query_dsl::methods::{FindDsl, SelectDsl};
use diesel::sql_types::BigInt;
use diesel::types::{FromSql, HasSqlType};

use diesel::prelude::*;

use serde::Serialize;

pub trait Rest {
    type Backend: diesel::backend::Backend + HasSqlType<<Self::Table as AsQuery>::SqlType>;
    type Conn: Connection<Backend = Self::Backend>;
    type Bridge: ConnectionRetriever<Self::Backend>;
    type Table: RunQueryDsl<Self::Conn> + QueryId + Table + HasTable<Table = Self::Table>;
    type QueryType: Queryable<<Self::Table as AsQuery>::SqlType, Self::Backend> + Serialize;
    type IdType;

    fn count_instances(conn: Self::Conn) -> String
    where
        <Self::Table as AsQuery>::Query:
            QueryId + QueryFragment<Self::Backend> + SelectDsl<CountStar>,
        Select<<Self::Table as AsQuery>::Query, CountStar>: RunQueryDsl<Self::Conn>
            + QueryId
            + QueryFragment<Self::Backend>
            + Query<SqlType = BigInt>,
        i64: FromSql<diesel::sql_types::BigInt, Self::Backend>,
        CountStar: Expression<SqlType = BigInt>,
    {
        let table = Self::Table::table();
        let count = QueryDsl::count(table).get_result::<i64>(&conn).unwrap();
        format!("{}", count)
    }

    fn retriever_count_instances() -> DbRetriever<Self::Backend, Self::Bridge> {
        DbRetriever::new()
    }

    fn get_by_pk((conn, id): (Self::Conn, Self::IdType)) -> JsonResponse<Self::QueryType>
    where
        Self::Table: FindDsl<Self::IdType>,
        Find<Self::Table, Self::IdType>: RunQueryDsl<Self::Conn>
            + QueryId
            + QueryFragment<Self::Backend>
            + Query<SqlType = <Self::Table as AsQuery>::SqlType>,
    {
        let table = Self::Table::table();
        let fetched = QueryDsl::find(table, id)
            .load::<Self::QueryType>(&conn)
            .unwrap()
            .remove(0);
        JsonResponse::new(fetched)
    }

    fn retrievers_get_by_pk() -> (
        DbRetriever<Self::Backend, Self::Bridge>,
        IndexedParamRetriever<Self::IdType>,
    ) {
        (DbRetriever::new(), IndexedParamRetriever::new(1))
    }
}

pub struct RestMarker;
impl<T> Template<RestMarker> for T
where
    T: Rest,
{
    fn mount_template(_mounter: impl Mounter) {}
}
