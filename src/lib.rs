#[cfg(test)]
async fn test() {
    use sqlx::PgPool;
    use testcontainers::{runners::AsyncRunner, ImageExt};
    use testcontainers_modules::postgres::Postgres;

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

#[tokio::test]
async fn test1() {
    test().await;
}
#[tokio::test]
async fn test2() {
    test().await;
}
#[tokio::test]
async fn test3() {
    test().await;
}
#[tokio::test]
async fn test4() {
    test().await;
}
#[tokio::test]
async fn test5() {
    test().await;
}
#[tokio::test]
async fn test6() {
    test().await;
}
#[tokio::test]
async fn test7() {
    test().await;
}
#[tokio::test]
async fn test8() {
    test().await;
}
#[tokio::test]
async fn test9() {
    test().await;
}
#[tokio::test]
async fn test10() {
    test().await;
}
#[tokio::test]
async fn test11() {
    test().await;
}
#[tokio::test]
async fn test12() {
    test().await;
}
#[tokio::test]
async fn test13() {
    test().await;
}
#[tokio::test]
async fn test14() {
    test().await;
}
#[tokio::test]
async fn test15() {
    test().await;
}
#[tokio::test]
async fn test16() {
    test().await;
}
#[tokio::test]
async fn test17() {
    test().await;
}
#[tokio::test]
async fn test18() {
    test().await;
}
#[tokio::test]
async fn test19() {
    test().await;
}
#[tokio::test]
async fn test20() {
    test().await;
}
#[tokio::test]
async fn test21() {
    test().await;
}
#[tokio::test]
async fn test22() {
    test().await;
}
#[tokio::test]
async fn test23() {
    test().await;
}
#[tokio::test]
async fn test24() {
    test().await;
}
#[tokio::test]
async fn test25() {
    test().await;
}
#[tokio::test]
async fn test26() {
    test().await;
}
#[tokio::test]
async fn test27() {
    test().await;
}
#[tokio::test]
async fn test28() {
    test().await;
}
#[tokio::test]
async fn test29() {
    test().await;
}
#[tokio::test]
async fn test30() {
    test().await;
}
#[tokio::test]
async fn test31() {
    test().await;
}
#[tokio::test]
async fn test32() {
    test().await;
}
#[tokio::test]
async fn test33() {
    test().await;
}
#[tokio::test]
async fn test34() {
    test().await;
}
#[tokio::test]
async fn test35() {
    test().await;
}
#[tokio::test]
async fn test36() {
    test().await;
}
#[tokio::test]
async fn test37() {
    test().await;
}
#[tokio::test]
async fn test38() {
    test().await;
}
#[tokio::test]
async fn test39() {
    test().await;
}
#[tokio::test]
async fn test40() {
    test().await;
}
#[tokio::test]
async fn test41() {
    test().await;
}
#[tokio::test]
async fn test42() {
    test().await;
}
#[tokio::test]
async fn test43() {
    test().await;
}
#[tokio::test]
async fn test44() {
    test().await;
}
#[tokio::test]
async fn test45() {
    test().await;
}
#[tokio::test]
async fn test46() {
    test().await;
}
#[tokio::test]
async fn test47() {
    test().await;
}
#[tokio::test]
async fn test48() {
    test().await;
}
#[tokio::test]
async fn test49() {
    test().await;
}
#[tokio::test]
async fn test50() {
    test().await;
}
#[tokio::test]
async fn test51() {
    test().await;
}
#[tokio::test]
async fn test52() {
    test().await;
}
#[tokio::test]
async fn test53() {
    test().await;
}
#[tokio::test]
async fn test54() {
    test().await;
}
#[tokio::test]
async fn test55() {
    test().await;
}
#[tokio::test]
async fn test56() {
    test().await;
}
#[tokio::test]
async fn test57() {
    test().await;
}
#[tokio::test]
async fn test58() {
    test().await;
}
#[tokio::test]
async fn test59() {
    test().await;
}
#[tokio::test]
async fn test60() {
    test().await;
}
#[tokio::test]
async fn test61() {
    test().await;
}
#[tokio::test]
async fn test62() {
    test().await;
}
#[tokio::test]
async fn test63() {
    test().await;
}
#[tokio::test]
async fn test64() {
    test().await;
}
#[tokio::test]
async fn test65() {
    test().await;
}
#[tokio::test]
async fn test66() {
    test().await;
}
#[tokio::test]
async fn test67() {
    test().await;
}
#[tokio::test]
async fn test68() {
    test().await;
}
#[tokio::test]
async fn test69() {
    test().await;
}
#[tokio::test]
async fn test70() {
    test().await;
}
#[tokio::test]
async fn test71() {
    test().await;
}
#[tokio::test]
async fn test72() {
    test().await;
}
#[tokio::test]
async fn test73() {
    test().await;
}
#[tokio::test]
async fn test74() {
    test().await;
}
#[tokio::test]
async fn test75() {
    test().await;
}
#[tokio::test]
async fn test76() {
    test().await;
}
#[tokio::test]
async fn test77() {
    test().await;
}
#[tokio::test]
async fn test78() {
    test().await;
}
#[tokio::test]
async fn test79() {
    test().await;
}
