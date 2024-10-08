use crate::domain::tiles::color::Color;
use crate::domain::tiles::number::Number as DomainNumber;

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct Number(i32);
// i32 because https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html#types

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "tile_sum_type")]
pub enum TileSumType {
    RegularTile,
    JokersWild,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "colornumber")]
struct ColorNumber {
    col: Color,
    num: Number,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "tile_type")]
struct TileType {
    regular_or_joker: TileSumType,
    colnum: ColorNumber,
}

pub fn convert_num(n: DomainNumber) -> Number {
    let foo = n.as_value().as_u16();
    let bar: i32 = foo as i32;
    Number(bar)
}

#[derive(Debug, sqlx::FromRow)]
pub struct ScratchRow {
    id: i32,
    num: Number,
}

#[cfg(test)]
mod basic_postgres_tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn connect_and_write() {
        // Connection Here
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:postgres@localhost/dev")
            .await
            .unwrap();

        // Write Just Simple Number with bind because color ful
        let my_num: Number = Number(12);
        println!("MyNum: {my_num:?}");
        let returned: ScratchRow =
            sqlx::query_as(r#"insert into rustikub.scratch (num) values ( $1 ) returning *"#)
                .bind(my_num)
                .fetch_one(&pool)
                .await
                .unwrap();
    }
}
