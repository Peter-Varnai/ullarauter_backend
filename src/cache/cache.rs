use crate::{models::{Bio_Exhibs_html, SidebarElement_eng_html, 
                    SidebarElement_de_html, SidebarExhibs},
            helpers::format_date};
use crate::services::{db_get_exhibitions, db_get_projects};
use sqlx::{Pool, Sqlite};
use std::collections::{HashMap, BTreeMap};
use crate::cache::{ BIO_EXHIBS_LOCK, SIDEBAR_LOCK, FRONTPAGE_EXHIBS };
use askama::Template;
use chrono::{NaiveDate, Utc};


pub async fn update_bio_exhibs_cache(db: &Pool<Sqlite>) {
    let exhibitions = db_get_exhibitions(db).await;
    let mut year_groups: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();

    for row in exhibitions {
        let year = &row[1][..4].to_string();

        let year_group = year_groups.get_mut(year);

        match year_group {
            Some(group) => {
                group.push(row);       
            }, None => {
                year_groups.insert(year.clone(), vec![row]);
            }
        }
    }


    let all_exhibitions: Vec<_> = year_groups.into_values().rev().collect(); 
    let rendered = Bio_Exhibs_html { all_exhibitions }.render().unwrap();

    let mut cache = BIO_EXHIBS_LOCK.write().unwrap();
    *cache = rendered;
    println!("Exhibition cache built!");
}


pub async fn update_sidebar_cashe(db: &Pool<Sqlite>) {
    let all_projects = db_get_projects(db).await;
    // let base_url = env::var("BASE_URL").unwrap_or("http://127.0.0.1:8080".to_string());
    let current_exhib = FRONTPAGE_EXHIBS.read().unwrap();

    let rendered_eng = SidebarElement_eng_html {
        all_projects: &all_projects,
        // base_url,
        current_exhib: &current_exhib,
    }.render().unwrap();

    let rendered_de = SidebarElement_de_html {
        all_projects: &all_projects,
        // base_url,
        current_exhib: &current_exhib, 
    }.render().unwrap();

    
    let mut sidebar_elems = HashMap::new();
    sidebar_elems.insert("eng".to_string(), rendered_eng);
    sidebar_elems.insert("de".to_string(), rendered_de);

    let mut cache =  SIDEBAR_LOCK.write().unwrap();
    *cache = sidebar_elems;
    println!("Sidebar Cache built!");
}


pub async fn update_sidebar_exhibs_cache(db: &Pool<Sqlite>) {
     let mut exhibitions = db_get_exhibitions(db).await;

    let now = Utc::now().date_naive();
    let mut current_exhib: Vec<Vec<String>> = Vec::new();

    for exhib in &mut exhibitions {
        // println!("exhib date: {}", exhib[2]);
        let naive_till = NaiveDate::parse_from_str(&mut exhib[2], "%Y%m%d").expect("Exhibition date conversion failed");
        if now <= naive_till {
            for (i, data) in exhib.iter_mut().enumerate() {
                if i == 1 || i == 2 { *data = format_date(data); }
            }
            current_exhib.push(exhib.clone());
        };
    }

    let rendered_exhib_sidebar = SidebarExhibs {current_exhib}.render().unwrap();
    let mut cache = FRONTPAGE_EXHIBS.write().unwrap();
    *cache =  rendered_exhib_sidebar;
    println!("Sidebar exhibs Cache built!");
}


pub async fn warm_cache(db:&Pool<Sqlite>) {
    update_sidebar_exhibs_cache(db).await;
    update_sidebar_cashe(db).await;
    update_bio_exhibs_cache(db).await;
}
