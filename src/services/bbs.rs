use crate::models::Bbs;
use crate::util::convert_to_shift_jis;
use actix_web::{get, web, HttpResponse, Responder};
use bytes::Bytes;
use htmlescape::encode_minimal;

#[get("/{bbs_path_name}/")]
pub async fn get_bbs_handler(web::Path(bbs_path_name): web::Path<String>) -> impl Responder {
    let bbs = Bbs::find_by_path_name(&bbs_path_name).await.unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(format!(
            "<html><head><title>{}</title></head><body><h1>{}</h1></body></html>",
            bbs.name, bbs.name
        ))
}

#[get("/bbsmenu.html")]
pub async fn get_bbs_menu_handler() -> impl Responder {
    let bbses = Bbs::find_shown().await.unwrap();

    match generate_bbsmenu_html(bbses) {
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
fn generate_bbsmenu_html(bbses: Vec<Bbs>) -> Option<Bytes> {
    if bbses.len() == 0 {
        return None;
    }

    let mut bbs_list = String::from(BBSMENU_HTML_TOP);
    let mut category = bbses[0].category.clone();
    bbs_list.push_str(&format!("<br><B>{}</B><br>\n", category));
    for bbs in bbses {
        if category != bbs.category {
            category = bbs.category;
            bbs_list.push_str(&format!("<br><br><B>{}</B><br><br>\n", category));
        }
        let name = encode_minimal(&bbs.name);
        bbs_list.push_str(format!("<A HREF=\"/{}\">{}</A><br>\n", bbs.path_name, name,).as_str());
    }
    bbs_list.push_str(BBSMENU_HTML_BOTTOM);
    let bbs_list_sjis = convert_to_shift_jis(bbs_list);
    Option::from(bbs_list_sjis)
}

const BBSMENU_HTML_TOP: &str = "
<HTML>
<HEAD>
<META http-equiv=\"Content-Type\" content=\"text/html; charset=Shift_JIS\">
<TITLE>BBS MENU for hyper2ch</TITLE>
</HEAD>
<BODY TEXT=\"#CC3300\" BGCOLOR=\"#FFFFFF\" link=\"#0000FF\" alink=\"#ff0000\" vlink=\"#660099\">
<font size=2>
";

const BBSMENU_HTML_BOTTOM: &str = "
</font>
</BODY>
</HTML>
";
