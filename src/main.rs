use slint::{Image, ModelRc, VecModel};
use std::{num::ParseFloatError, path::{Path, PathBuf}, rc::Rc};

slint::include_modules!();

fn type_to_img(r#type: FurnitureItemTypes) -> Box<Path> {
    match r#type {
        FurnitureItemTypes::Room => PathBuf::from("./images/Room.png").into_boxed_path(),
        FurnitureItemTypes::Bed => PathBuf::from("./images/Bed.png").into_boxed_path(),
        FurnitureItemTypes::Window => PathBuf::from("./images/Window.png").into_boxed_path(),
        FurnitureItemTypes::Wall => PathBuf::from("./images/Wall.png").into_boxed_path(),
        FurnitureItemTypes::Table => PathBuf::from("./images/Table.png").into_boxed_path(),
        FurnitureItemTypes::Chair => PathBuf::from("./images/Chair.png").into_boxed_path(),
        FurnitureItemTypes::Closet => PathBuf::from("./images/Closet.png").into_boxed_path(),
    }
}

fn check_value_incorrect(val_res_: &(&str, &str, Result<f32, ParseFloatError>), index: usize, limits: Option<&(f32, f32)>) -> bool {
    let (_, _, val_res) = val_res_;
    if val_res.as_ref().is_err() {
        return true
    } else if val_res.as_ref().is_ok_and(|&x| x<=0.0) {
        return true
    } else if limits.is_some() && index==0 && val_res.as_ref().is_ok_and(|&x| x>limits.unwrap().0){
        return true
    } else if limits.is_some() && index==1 && val_res.as_ref().is_ok_and(|&x| x>limits.unwrap().1){
        return true
    } else { return false }
}

fn catch_create_error(val_res_: &(&str, &str, Result<f32, ParseFloatError>)) -> String {
    let (content, name, val_res) = val_res_;
    if val_res.as_ref().is_err() {
        match content.is_empty() {
            true => format!("Error: {name} shouldn't be empty!"),
            false => format!("Error: {name} should be a number!")
        }
    } else if val_res.as_ref().is_ok_and(|&x| x<=0.0) {
        format!("Error: {name} should be positive!")
    } else {
        format!("Error: {name} exceeds room limits!")
    } 
}

fn which_errors(values: [(&str, &str, Result<f32, ParseFloatError>);2], limits: Option<&(f32, f32)>) -> Option<Vec<String>> {
    let output = values.iter().enumerate()
    .filter(|&(i, x)| check_value_incorrect(x, i, limits))
    .map(|(_, x)| catch_create_error(x))
    .collect::<Vec<String>>();
    if output.is_empty() {None} else {Some(output)}
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let mut items_list:Vec<FurnitureItemStats> = Vec::new();
    let mut items_properties_list:Vec<[f32;6]> = Vec::new();
    let mut real_values_stored:Vec<(f32, f32)> = Vec::new();

    ui.on_new_item(move |t, x, y| {
        let ui = ui_handle.unwrap();
        let length = x.trim().replace(",", ".");
        let width = y.trim().replace(",", ".");
        let value_checker = [
            (length.as_str(), "length", length.parse::<f32>()), 
            (width.as_str(), "width", width.parse::<f32>())
            ];

        if let Some(recieved_errors) = which_errors(value_checker, real_values_stored.get(0)) {    
            ui.set_how_many_errors_did_i_get((recieved_errors.len() as i32).into());
            ui.set_hmmm_which_error_did_i_get(recieved_errors.join("\n").into());
        } else { 
       
        let (item_length_real, item_width_real) = (length.parse::<f32>().unwrap(),width.parse::<f32>().unwrap());
        let gmtr_props:[f32;6]; /* horizontal, vertical, min_x, max_x, min_y, max_y */
        let ratio:f32;
            
        let item_length:f32; let item_width:f32;
        if t==FurnitureItemTypes::Room {
            ratio = item_length_real/item_width_real; 
            if ratio>=1.0 {
                item_length = 400.0; item_width = 400.0/ratio;
            } else {
                item_length = 400.0*ratio; item_width = 400.0;
            }
            gmtr_props = [item_length, item_width, (480.0-item_length)/2.0, (480.0-item_length)/2.0, (480.0-item_width)/2.0, (480.0-item_width)/2.0,];

            real_values_stored.clear();
            items_properties_list.clear();
            items_list.clear();
        } else { /* Уже есть Room */
            let (room_length_real, room_width_real) = *real_values_stored.get(0).unwrap(); 
            ratio = room_length_real/room_width_real;
            if ratio>=1.0 {
                item_length = 400.0*item_length_real/room_length_real;
                item_width = 400.0*item_width_real/room_width_real/ratio;
            } else {
                item_length = 400.0*item_length_real/room_length_real*ratio;
                item_width = 400.0*item_width_real/room_width_real;
            }
            let room_props = *items_properties_list.get(0).unwrap();
            let room_x_offset = room_props[2];
            let room_y_offset = room_props[4];
            gmtr_props = [item_length, item_width, 
                          room_x_offset, (room_x_offset-item_length+room_props[0]), 
                          room_y_offset, (room_y_offset-item_width+room_props[1])];
        }
        let item = FurnitureItemStats{
            geometry_properties: gmtr_props.into(),
            r#type: t,
            img: Image::load_from_path(&type_to_img(t)).unwrap()
        };
        real_values_stored.push((item_length_real, item_width_real));
        items_properties_list.push(gmtr_props);
        items_list.push(item);

        let new_active_items = Rc::new(VecModel::from(items_list.clone()));
        ui.set_active_items(ModelRc::from(new_active_items));

    }});

    ui.run()
}
