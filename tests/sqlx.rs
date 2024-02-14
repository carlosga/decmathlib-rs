#![allow(dead_code)]

#[cfg(feature = "sqlx")]
#[sqlx::test]
async fn sqlx_001() -> sqlx::Result<()> {
    // Create a connection pool to your PostgreSQL database
    let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost/postgres").await?;

    // Drop the test table
    sqlx::query(
        r#"DROP TABLE IF EXISTS test_data "#,
    )
    .execute(&pool)
    .await?;

    // Create the test table
    sqlx::query(
        r#"
        CREATE TABLE test_data (
            id SERIAL PRIMARY KEY,
            value DECIMAL(10, 2)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Insert sample data into the table using a parameterized query
    // let value: d128 = decmathlib_rs::dec128!(123.45);
    let value: f64 = 100000.1234f64;
    sqlx::query(r#"INSERT INTO test_data (value) VALUES ($1)"#)
        .bind(value)
        .execute(&pool)
        .await?;

    #[derive(sqlx::FromRow)]
    struct TestData { id: i32, value: decmathlib_rs::d128::d128 }

    // Fetch data from the table
    let result = sqlx::query_as::<_, TestData>("SELECT * FROM test_data")
        .fetch_one(&pool)
        .await?;

    // Access the data (assuming "value" is a column in your table)
    // assert_eq!(first_value, 123.45);

    // Add more assertions as needed

    Ok(())
}