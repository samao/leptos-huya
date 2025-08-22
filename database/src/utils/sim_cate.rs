use crate::models::SimCate as DbSimCate;
use crate::schema::sim_cate::dsl::*;
use diesel::prelude::*;
use models::SimCate as ModelSimCate;

pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<ModelSimCate>, String> {
    let sim_cates = sim_cate
        .select(DbSimCate::as_select())
        .get_results(conn)
        .map_err(|e| format!("{:?}", e.to_string()))?;
    Ok(sim_cates
        .into_iter()
        .map(|cate| cate.into())
        .collect::<Vec<_>>())
}
