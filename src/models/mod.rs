pub mod data;
pub mod template_defs;
pub mod forms;
// pub use db_def;
pub use data::{AppState,AdminQuery, DeleteExhibitionRequest, 
    DeleteProjectRequest, DeleteBackgroundRequest,
    SetLangRequest};
pub use template_defs::{Index, Contact, Bio,  Bio_Exhibs_html, SidebarElement_eng_html, 
    SidebarElement_de_html, Project, LoginTemplate, AdminTemplate, SidebarExhibs};
pub use forms::{ LoginForm, BioForm, CvUploadForm, ExhibitionForm, ImgCommentForm };

