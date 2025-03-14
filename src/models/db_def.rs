// pub mod definitions {
//
//     use sqlx::FromRow;
//     use serde::Serialize;
//     #[derive(Serialize, FromRow)]
//     struct Exhibition {
//         id: String,
//         title: String,
//         fromm: String,
//         till: String,
//         location: String,
//         link: String,
//         title_de: String,
//     }
//
//
//     #[derive(Serialize, FromRow)]
//     struct FpProjects {
//         id: String,
//         image: String,
//         title: String,
//         title_de: String,
//         pictures_folder: String,
//     }
//
//
//     #[derive(Serialize, FromRow)]
//     struct ImageComments {
//         id: String,
//         picture_folder: String,
//         file_name: String,
//         eng_comment: String,
//         de_comment: String,
//     }
//
//
//     #[derive(Serialize, FromRow)]
//     struct PersonalDetails {
//         id: String,
//         bio_eng: String,
//         bio_de: String,
//         pfp_address: String,
//         cv_eng_address: String,
//         cv_de_address: String,
//     }
//
//
//     #[derive(Serialize, FromRow)]
//     struct Project {
//         id: String,
//         title: String,
//         text_eng: String,
//         text_de: String,
//         video: String,
//         pictures: String,
//         main_picture: String,
//         name_de: String,
//         pictures_folder: String,
//         date: String,
//     }
// }
