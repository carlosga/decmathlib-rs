/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for full license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018 Intel Corp.                        */
/* -------------------------------------------------------------------------------------------------- */

#[cfg(feature = "sqlx")]
use std::str::FromStr;

#[macro_export]
macro_rules! sqlx_test {
    ($name:ident, sqlx_decimal, $precision:expr, $scale:expr, $input1:expr) => {
        #[cfg(feature = "sqlx")]
        #[sqlx::test]
        #[allow(dead_code)]
        async fn $name() -> sqlx::Result<()> {
            let table_name = stringify!($name);

            // Create a connection pool to your PostgreSQL database
            let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost/postgres").await?;

            // Drop the test table
            let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
            sqlx::query(&drop_query)
                .execute(&pool)
                .await?;

            // Create the test table
            let create_query = format!("CREATE TABLE {} (id SERIAL PRIMARY KEY, value DECIMAL({}, {}))", table_name, $precision, $scale);
            sqlx::query(&create_query)
                .execute(&pool)
                .await?;

            // Insert sample data into the table using a parameterized query
            let dec1: decmathlib_rs::d128::d128 = decmathlib_rs::d128::d128::from_str($input1).unwrap();
            let insert_query = format!("INSERT INTO {} (value) VALUES ($1)", table_name);
            sqlx::query(&insert_query)
                .bind(dec1)
                .execute(&pool)
                .await?;

            #[derive(sqlx::FromRow)]
            struct TestData { id: i32, value: decmathlib_rs::d128::d128 }

            // Fetch data from the table
            let select_query = format!("SELECT * FROM {}", table_name);
            let result = sqlx::query_as::<_, TestData>(&select_query)
                .fetch_one(&pool)
                .await?;

            assert_eq!(dec1, result.value);

            Ok(())
        }
    };
}

// sqlx_test!(sqlx_decimal_001, sqlx_decimal, 18, 6, "100000.122334");
// sqlx_test!(sqlx_decimal_002, sqlx_decimal, 18, 6, "+9878987.5679766895E0");
// sqlx_test!(sqlx_decimal_005, sqlx_decimal, 18, 6, "9.999999999999999999999999999999999E6144");
sqlx_test!(sqlx_decimal_006, sqlx_decimal, 34, 6, "1");
sqlx_test!(sqlx_decimal_007, sqlx_decimal, 34, 6, "1E+4");
sqlx_test!(sqlx_decimal_008, sqlx_decimal, 34, 6, "1E+8");
sqlx_test!(sqlx_decimal_009, sqlx_decimal, 34, 6, "1E+12");
sqlx_test!(sqlx_decimal_010, sqlx_decimal, 34, 6, "1E+16");
sqlx_test!(sqlx_decimal_011, sqlx_decimal, 34, 6, "1E+20");
sqlx_test!(sqlx_decimal_012, sqlx_decimal, 34, 6, "1E+24");
// sqlx_test!(sqlx_decimal_013, sqlx_decimal, 34, 6, "1E+28");
