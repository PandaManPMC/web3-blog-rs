use axum::extract::Request;

pub fn set_user_to_req(request: &mut Request, user: base::model::blog_author::BlogAuthorModel){
    request.headers_mut().insert("x-user-name", user.user_name.parse().unwrap());
    request.headers_mut().insert("x-user-id", user.id.into());
    request.headers_mut().insert("x-user-pen_name", user.pen_name.parse().unwrap());
}

