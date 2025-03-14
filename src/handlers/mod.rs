pub mod pub_handlers;
pub mod admin_handlers;
pub use pub_handlers::{ index, contact, bio, set_language, project, session_info, admin };
pub use admin_handlers::{ login, logout, create_project, delete_background,
                        upload_background, add_exhibition, 
                        delete_exhibition, delete_project, update_pfp,
                        update_cv, update_bio, get_project,
                        add_image_comment};
