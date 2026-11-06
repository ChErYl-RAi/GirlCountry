extern crate nfd;
use std::env;
use nfd::Response;

pub fn load(bwee: bool) -> String {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let direction = exe_dir.join("assets").join("tmp").join("plain.gcountry");
    let tdirection = exe_dir.join("assets").join("tmp");
    let tdir = direction.to_str().unwrap().to_owned();
    let ttdir = tdirection.to_str().unwrap().to_owned();
    let result: Response;
    if bwee{
      result = nfd::open_file_dialog(Some("gcountry"), Some(&ttdir)).unwrap();
    }else{
      result = nfd::open_file_dialog(Some("gcountry"), None).unwrap();
    }

    match result {
        Response::Okay(file_path) => file_path,
        Response::OkayMultiple(files) => files[0].clone(),
        Response::Cancel => tdir,
    }
}


pub fn saveas() -> String {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let result = nfd::open_save_dialog(Some("gcountry"), Some(exe_dir.to_str().unwrap())).unwrap();

    match result {
        Response::Okay(file_path) => file_path,
        Response::OkayMultiple(files) => files[0].clone(),
        Response::Cancel => "exe_path".to_string(),
    }
}