use crate::diesel::prelude::*;
use crate::models_diesel::*;
use crate::schema::bbses::dsl::*;
use crate::util::convert_to_shift_jis;
use actix_web::{get, web, HttpResponse, Responder};
use bytes::Bytes;
use dotenv_codegen::dotenv;
use htmlescape::encode_minimal;

#[get("/{bbs_path_name}/")]
pub async fn get_bbs_handler(web::Path(bbs_path_name): web::Path<String>) -> impl Responder {
    let conn = establish_connection();

    let result = bbses
        .filter(path_name.eq(bbs_path_name))
        .first::<Bbs>(&conn)
        .expect("Error");

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(format!(
            "<html><head><title>{}</title></head><body><h1>{}</h1></body></html>",
            result.name, result.name
        ))
}

#[get("/bbsmenu.html")]
pub async fn get_bbs_menu_handler() -> impl Responder {
    let conn = establish_connection();

    let result = bbses
        .filter(hidden.eq(false))
        .load::<Bbs>(&conn)
        .expect("Error");

    match generate_bbsmenu_html(result) {
        Some(bbsmenu_html) => HttpResponse::Ok()
            .content_type("text/html; charset=Shift_JIS")
            .body(bbsmenu_html),
        None => HttpResponse::Ok()
            .content_type("text/plain; charset=Shift_JIS")
            .body("No BBS"),
    }
}

/// Generate bbsmenu.txt for listing BBSes
/// TODO: Fix problem that 2ch browser cannot load
fn generate_bbsmenu_html(bbs_vec: Vec<Bbs>) -> Option<Bytes> {
    if bbs_vec.len() == 0 {
        return None;
    }

    let backend_address = dotenv!("BACKEND_ADDRESS");

    let mut bbs_list = String::from(BBSMENU_HTML_TOP);
    let mut cat = bbs_vec[0].category.clone();
    bbs_list.push_str(&format!(
        "<h1><a href={}>hyper2ch</a></h1>\n<h2>BBS MENU</h2>\n<ul>\n",
        backend_address
    ));
    bbs_list.push_str(&format!("<li><h3>{}</h3></li>\n<ul>\n", cat));
    for b in bbs_vec {
        if cat != b.category {
            cat = b.category;
            bbs_list.push_str(&format!("</ul>\n<li><h3>{}</h3></li>\n<ul>\n", cat));
        }
        let encoded_name = encode_minimal(&b.name);
        bbs_list.push_str(
            format!(
                "<li><a href=\"{}/{}/\">{}</a></li>\n",
                backend_address, b.path_name, encoded_name
            )
            .as_str(),
        );
    }
    bbs_list.push_str(BBSMENU_HTML_BOTTOM);
    let bbs_list_sjis = convert_to_shift_jis(bbs_list);
    Option::from(bbs_list_sjis)
}

const BBSMENU_HTML_TOP: &str = "<html>\n<head>\n<meta http-equiv=\"content-type\" content=\"text/html; charset=shift_jis\">\n<title>BBS MENU for hyper2ch</title>\n</head>\n<body>";
const BBSMENU_HTML_BOTTOM: &str = "</ul>\n</body>\n</html>";
