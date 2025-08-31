#![allow(dead_code)]
#![allow(unused)]

use anyhow::anyhow;
use database::establish_connection;
use database::schema::rooms::hot;
use database::schema::tags::title;
use database::schema::vod_cate;
use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use serde::Deserialize;
use tokio::fs::read_to_string;
use tracing::info;

use database::models::{User, Vod as DbVod, VodCate};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = &mut establish_connection();
    let json_str = read_to_string("database/data/vods-api.json").await?;
    let all: VodPage = serde_json::from_str(&json_str).unwrap();
    all.save(conn)?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct VodUser {
    name: String,
    avatar: String,
}

#[derive(Debug, Deserialize)]
struct Vod {
    title: String,
    img_url: String,
    hots: i32,
    duration: String,
    owner: VodUser,
}

impl Vod {
    fn duration_as_sec(&self) -> i32 {
        let mut index = 0;
        let times = self
            .duration
            .split(":")
            .map(|item| item.parse::<i32>().unwrap_or_default())
            .collect::<Vec<_>>()
            .iter()
            .rfold(0, |total, acc| {
                let next_total = total + acc * 60i32.pow(index);
                index += 1;
                next_total
            });
        times
    }
}

#[derive(Debug, Deserialize)]
struct VodSet {
    title: String,
    cover: String,
    tags: Vec<String>,
    list: Vec<Vod>,
    rank: Vec<Vod>,
}

#[derive(Debug, Deserialize)]
struct Site {
    title: String,
    list: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Banner {
    title: String,
    cover: String,
}

#[derive(Debug, Deserialize)]
struct VodPage {
    banners: Vec<Banner>,
    sites: Vec<Site>,
    aggregations: Vec<VodSet>,
    recommends: Vec<Vod>,
}

#[derive(Debug)]
struct FullVod {
    pub id: i32,
    pub img_url: String,
    pub duration: i32,
    pub user: User,
    pub title: String,
    pub vod_cate_id: i32,
    pub hots: i32,
}

impl From<(DbVod, User)> for FullVod {
    fn from(
        (
            DbVod {
                id,
                img_url,
                duration,
                title: vod_title,
                vod_cate_id,
                hots,
                ..
            },
            user,
        ): (DbVod, User),
    ) -> Self {
        Self {
            id,
            img_url,
            duration,
            title: vod_title,
            vod_cate_id,
            hots,
            user,
        }
    }
}

impl VodPage {
    fn save(&self, conn: &mut SqliteConnection) -> anyhow::Result<()> {
        conn.transaction(|conn| {
            self.save_banner(conn)?;
            self.save_sites(conn)?;
            let cateids = self.save_aggregations(conn)?;
            self.save_recommends(conn, cateids)?;
            self.get_hot_vods(conn)?;
            Err(anyhow!("手动回滚"))
        })
    }

    fn get_hot_vods(&self, conn: &mut SqliteConnection) -> anyhow::Result<Vec<FullVod>> {
        use database::schema::users::dsl as user_dsl;
        use database::schema::vods::dsl::*;
        //inner_join 必须有， left_join 可能
        let hot_vods = vods
            .inner_join(user_dsl::users.on(user_id.eq(user_dsl::id)))
            .limit(6)
            .order(hots.desc())
            .select((DbVod::as_select(), User::as_select()))
            .load::<(DbVod, User)>(conn)?;
        let result = hot_vods.into_iter().map(|item| item.into()).collect();
        info!("热度最高的6个: {:?}", result);
        Ok(result)
    }

    // fn get_vod_cates(&self, conn: &mut SqliteConnection, )

    fn save_banner(&self, conn: &mut SqliteConnection) -> anyhow::Result<()> {
        info!("保存Banner: {} 个", self.banners.len());
        Ok(())
    }
    fn save_sites(&self, conn: &mut SqliteConnection) -> anyhow::Result<()> {
        info!("保存sites: {} 个", self.sites.len());
        Ok(())
    }
    fn save_aggregations(&self, conn: &mut SqliteConnection) -> anyhow::Result<Vec<i32>> {
        info!("保存aggregations: {} 个", self.aggregations.len());
        let mut cateids = vec![];
        for vod_set in self.aggregations.iter() {
            let cat_set = self.save_cate(conn, vod_set.title.clone(), vod_set.cover.clone())?;
            cateids.push(cat_set.id);
            if vod_set.tags.len() > 1 {
                //save sub_cate
                self.save_sub_cate(conn, cat_set.id, &vod_set.tags)?;
            }
            for vod in vod_set.list.iter() {
                self.save_vod(conn, vod, cat_set.id)?;
            }
            for vod in vod_set.rank.iter() {
                self.save_vod(conn, vod, cat_set.id)?;
            }
        }
        Ok(cateids)
    }

    fn save_sub_cate(
        &self,
        conn: &mut SqliteConnection,
        parent_id: i32,
        tags: &Vec<String>,
    ) -> anyhow::Result<()> {
        use database::models::SubCate;
        use database::schema::sub_cate::dsl::*;
        info!("创建子目录: {tags:?}");
        for tag in tags {
            diesel::insert_into(sub_cate)
                .values((vod_cate_id.eq(parent_id), cate_name.eq(tag)))
                .execute(conn)?;
        }
        Ok(())
    }

    fn save_cate(
        &self,
        conn: &mut SqliteConnection,
        cname: String,
        cimg_url: String,
    ) -> anyhow::Result<VodCate> {
        use database::schema::vod_cate::dsl::*;
        info!("创建Cate: {cname}");
        let vcate = diesel::insert_into(vod_cate)
            .values((cate_name.eq(cname), img_url.eq(cimg_url)))
            .returning(VodCate::as_returning())
            .get_result(conn)?;
        Ok(vcate)
    }
    fn save_recommends(
        &self,
        conn: &mut SqliteConnection,
        cateids: Vec<i32>,
    ) -> anyhow::Result<()> {
        info!("保存recommends: {} 个", self.recommends.len());
        for (vod, cid) in self.recommends.iter().zip(cateids.iter().cycle()) {
            self.save_vod(conn, vod, *cid)?;
        }
        info!("保存推荐视频完成");
        Ok(())
    }
    fn save_vod(&self, conn: &mut SqliteConnection, vod: &Vod, cid: i32) -> anyhow::Result<()> {
        use database::models::Vod;
        use database::schema::{
            sub_cate::dsl as sub_cate_dsl, users::dsl as users_dsl, vod_cate::dsl as cate_dsl,
            vods::dsl as vod_dsl,
        };
        use database::users::create;

        let user = create(
            conn,
            vod.owner.name.chars().take(10).collect(),
            Some(vod.owner.avatar.clone()),
            "123456".to_string(),
            None,
        )?;
        let vod_db = diesel::insert_into(vod_dsl::vods)
            .values((
                vod_dsl::title.eq(vod.title.clone()),
                vod_dsl::img_url.eq(vod.img_url.clone()),
                vod_dsl::duration.eq(vod.duration_as_sec()),
                vod_dsl::user_id.eq(user.id),
                vod_dsl::vod_cate_id.eq(cid),
                vod_dsl::hots.eq(vod.hots),
            ))
            .returning(Vod::as_returning())
            .execute(conn)?;
        if vod_db != 0 {
            info!("保存视频成功");
        }
        Ok(())
    }
}
