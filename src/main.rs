use sqlx::PgPool;
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::postgres::Postgres;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    for i in 0..50 {
        println!("{i}");
        let container2 = Postgres::default()
            .with_tag("16")
            .start()
            .await
            .unwrap();

        let container1 = Postgres::default()
            .with_tag("16")
            .start()
            .await
            .unwrap();

        let connection_string = &format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            container1.get_host().await.unwrap(),
            container1.get_host_port_ipv4(5432).await.unwrap(),
        );


        let pool = PgPool::connect(connection_string).await.unwrap();

        sqlx::query("select 1").execute(&pool).await.unwrap();

        drop(container1);
        drop(container2);
    }
}
