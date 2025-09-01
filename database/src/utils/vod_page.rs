use crate::models::{User as DbUser, Vod as DbVod, VodCate as DbVodCate};
use diesel::prelude::*;
use diesel::{Connection, SqliteConnection};
use models::{Banner, FullVod, Site, Vod, VodSet, VodUser};
use serde::Deserialize;
use tracing::info;

pub fn duration_as_string(duration_sec: i64) -> String {
    let duration = chrono::Duration::seconds(duration_sec);
    let mins = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    match duration.num_hours() {
        0 => format!("{:02}:{:02}", mins, seconds),
        hours => format!("{:02}:{:02}:{:02}", hours, mins, seconds),
    }
}

#[derive(Debug, Deserialize)]
pub struct VodPage {
    pub banners: Vec<Banner>,
    pub sites: Vec<Site>,
    pub aggregations: Vec<VodSet>,
    pub recommends: Vec<Vod>,
}

pub fn get_hot_vods(conn: &mut SqliteConnection) -> anyhow::Result<Vec<FullVod>> {
    use crate::schema::users::dsl as user_dsl;
    use crate::schema::vods::dsl::*;
    //inner_join 必须有， left_join 可能
    let hot_vods = vods
        .inner_join(user_dsl::users.on(user_id.eq(user_dsl::id)))
        .limit(9)
        .order(hots.desc())
        .select((DbVod::as_select(), DbUser::as_select()))
        .load::<(DbVod, DbUser)>(conn)?;
    let result: Vec<FullVod> = hot_vods
        .into_iter()
        .map(|(item, user)| FullVod {
            id: item.id,
            img_url: item.img_url,
            duration: item.duration,
            user: VodUser {
                name: user.user_name,
                avatar: user.avatar,
            },
            title: item.title,
            vod_cate_id: item.vod_cate_id,
            hots: item.hots,
        })
        .collect();
    info!("热度视频: {:?}", result.len());
    Ok(result)
}
pub fn get_cate_vods(conn: &mut SqliteConnection) -> anyhow::Result<Vec<VodSet>> {
    use crate::schema::{
        sub_cate::dsl as sub_cate_dsl, vod_cate::dsl as vod_cate_dsl, vods::dsl as vods_dsl,
    };
    use crate::users::get_user;

    let mut cate_sets = vec![];

    let vod_cates = vod_cate_dsl::vod_cate
        .select(DbVodCate::as_select())
        .get_results(conn)?;
    for cate in vod_cates {
        let tags = sub_cate_dsl::sub_cate
            .filter(sub_cate_dsl::vod_cate_id.eq(cate.id))
            .select(sub_cate_dsl::cate_name)
            .get_results(conn)?;

        let cate_vods = vods_dsl::vods
            .limit(8)
            .filter(vods_dsl::vod_cate_id.eq(cate.id))
            .select(DbVod::as_select())
            .get_results(conn)?;

        let mut users = vec![];
        for video in cate_vods.iter() {
            let user = get_user(conn, video.user_id)?;
            users.push(user);
        }

        let cat_vods_rank = vods_dsl::vods
            .limit(7)
            .order(vods_dsl::hots.desc())
            .filter(vods_dsl::vod_cate_id.eq(cate.id))
            .select(DbVod::as_select())
            .get_results(conn)?;

        let mut rank_users = vec![];
        for video in cat_vods_rank.iter() {
            let user = get_user(conn, video.user_id)?;
            rank_users.push(user);
        }
        cate_sets.push(VodSet {
            title: cate.cate_name,
            cover: cate.img_url,
            tags: tags,
            list: cate_vods
                .into_iter()
                .zip(users)
                .map(|(item, user)| Vod {
                    title: item.title,
                    img_url: item.img_url,
                    hots: item.hots,
                    duration: duration_as_string(item.duration as i64),
                    owner: VodUser {
                        name: user.user_name,
                        avatar: user.avatar,
                    },
                })
                .collect(),
            rank: cat_vods_rank
                .iter()
                .zip(rank_users)
                .map(|(item, user)| Vod {
                    title: item.title.clone(),
                    img_url: item.img_url.clone(),
                    hots: item.hots,
                    duration: duration_as_string(item.duration as i64),
                    owner: VodUser {
                        name: user.user_name,
                        avatar: user.avatar,
                    },
                })
                .collect(),
        });
    }
    info!("读取分类视频数：{:#?}", cate_sets.len());

    Ok(cate_sets)
}

impl VodPage {
    pub fn save(&self, conn: &mut SqliteConnection) -> anyhow::Result<()> {
        conn.transaction(|conn| {
            self.save_banner(conn)?;
            self.save_sites(conn)?;
            let cateids = self.save_aggregations(conn)?;
            self.save_recommends(conn, cateids)?;
            Ok(())
        })
    }

    fn save_banner(&self, _conn: &mut SqliteConnection) -> anyhow::Result<()> {
        info!("保存Banner: {} 个", self.banners.len());
        Ok(())
    }
    fn save_sites(&self, _conn: &mut SqliteConnection) -> anyhow::Result<()> {
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
        use crate::schema::sub_cate::dsl::*;
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
    ) -> anyhow::Result<DbVodCate> {
        use crate::schema::vod_cate::dsl::*;
        info!("创建Cate: {cname}");
        let vcate = diesel::insert_into(vod_cate)
            .values((cate_name.eq(cname), img_url.eq(cimg_url)))
            .returning(DbVodCate::as_returning())
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
        use crate::schema::vods::dsl as vod_dsl;
        use crate::users::create as create_user;

        let user = create_user(
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
            .returning(DbVod::as_returning())
            .execute(conn)?;
        if vod_db != 0 {
            info!("保存视频成功");
        }
        Ok(())
    }
}
