// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod demos
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Demos
{ pub id : uuid::Uuid,pub name : String,pub created_at : time::OffsetDateTime,}pub struct DemosBorrowed<'a> { pub id : uuid::Uuid,pub name : &'a str,pub created_at : time::OffsetDateTime,}
impl<'a> From<DemosBorrowed<'a>> for Demos
{
    fn from(DemosBorrowed { id,name,created_at,}: DemosBorrowed<'a>) ->
    Self { Self { id,name: name.into(),created_at,} }
}pub struct DemosQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> DemosBorrowed,
    mapper: fn(DemosBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> DemosQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(DemosBorrowed) -> R) ->
    DemosQuery<'a,C,R,N>
    {
        DemosQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn demos() -> DemosStmt
{ DemosStmt(cornucopia_async::private::Stmt::new("select * from demos")) } pub struct
DemosStmt(cornucopia_async::private::Stmt); impl DemosStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> DemosQuery<'a,C,
Demos, 0>
{
    DemosQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { DemosBorrowed { id: row.get(0),name: row.get(1),created_at: row.get(2),} }, mapper: |it| { <Demos>::from(it) },
    }
} }}}