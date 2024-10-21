use std::error::Error;

use bytes::BytesMut;

use uuid::Uuid;

use postgres_protocol::types;
use tokio_postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};

use crate::Uuid32;

impl<'a> FromSql<'a> for Uuid32 {
    fn from_sql(_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let bytes = types::uuid_from_sql(raw)?;
        Ok(Uuid::from_bytes(bytes).into())
    }

    accepts!(UUID);
}

impl ToSql for Uuid32 {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        types::uuid_to_sql(*self.0.as_bytes(), out);
        Ok(IsNull::No)
    }

    accepts!(UUID);
    to_sql_checked!();
}
