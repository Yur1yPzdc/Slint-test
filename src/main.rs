use slint::{Image, ModelRc, VecModel};
use std::{num::ParseFloatError, path::{Path, PathBuf}, rc::Rc};

slint::include_modules!();

fn type_to_img(r#type: FurnitureItemTypes) -> Box<Path> {
    match r#type {
        FurnitureItemTypes::Room => PathBuf::from("./images/Room.png").into_boxed_path(),
        FurnitureItemTypes::Bed => PathBuf::from("./images/Bed.png").into_boxed_path()
    }
}

fn check_value_correct(val_res_: &(&str, &str, Result<f32, ParseFloatError>), show_prints: bool) -> f32 {
    let (content, name, val_res) = val_res_;
    if let Some(_) = val_res.as_ref().err() {
        if show_prints {
            match content.is_empty() {
                true => println!("Error: {name} shouldn't be empty!"),
                false => println!("Error: {name} should be a number!")
            }
        }
        return -1.0
    } else if val_res.as_ref().is_ok_and(|x| *x<=0.0) {
        if show_prints {println!("Error: {name} should be positive!");}
        return -1.0
    } else {
        return *val_res.as_ref().unwrap()
    }
}

fn catch_value_error(val_res_: &(&str, &str, Result<f32, ParseFloatError>)) -> String {
    let (content, name, val_res) = val_res_;
    if let Some(_) = val_res.as_ref().err() {
        match content.is_empty() {
            true => format!("Error: {name} shouldn't be empty!"),
            false => format!("Error: {name} should be a number!")
        }
    } else {
        format!("Error: {name} should be positive!")
    } 
}

fn check_all_value_correct(values: Vec<(&str, &str, Result<f32, ParseFloatError>)>) -> bool {
    values.iter().map(|x| check_value_correct(x, false)).fold(true, |i, x| i && (x>0.0))
}

fn which_errors(values: Vec<(&str, &str, Result<f32, ParseFloatError>)>) -> Vec<String> {
    values.iter()
    .filter(|x| check_value_correct(*x, false)==-1.0)
    .map(|x| catch_value_error(x))
    .collect()
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let mut items_props_list:Vec<[f32;6]> = Vec::new();
    let mut items_list:Vec<FurnitureItemStats> = Vec::new();
    let mut real_values_stored:Vec<(f32, f32)> = Vec::new();

    ui.on_new_item(move |t, x, y| {
        let ui = ui_handle.unwrap();
        let length = x.trim().replace(",", ".");
        let width = y.trim().replace(",", ".");
        let value_checker = [
            (&length[..], stringify!(length), length.parse::<f32>()), 
            (&width[..], stringify!(width), width.parse::<f32>())
            ];

        if !check_all_value_correct(value_checker.to_vec())
        {
            let recieved_errors = which_errors(value_checker.to_vec());
            ui.set_how_many_errors_did_i_get((recieved_errors.len() as i32).into());
            ui.set_hmmm_which_error_did_i_get(recieved_errors.join("\n").into());
        } else { 
       
        let (item_length_real, item_width_real) = (length.parse::<f32>().unwrap(),width.parse::<f32>().unwrap());
        let gmtr_props:[f32;6]; /* height, width, min_x, max_x, min_y, max_y */
        let ratio:f32;
            
        let item_length:f32; let item_width:f32;
        if t==FurnitureItemTypes::Room {
            ratio = item_length_real/item_width_real; 
            if ratio>=1.0 {
                item_length = 400.0; item_width = 400.0/ratio;
            } else {
                item_length = 400.0*ratio; item_width = 400.0;
            }
            gmtr_props = [item_length, item_width, (480.0-item_width)/2.0, (480.0-item_width)/2.0, (480.0-item_length)/2.0, (480.0-item_length)/2.0,];

            real_values_stored.clear();
            items_props_list.clear();
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
            let room_props = *items_props_list.get(0).unwrap();
            let room_x_offset = room_props[2];
            let room_y_offset = room_props[4];
            gmtr_props = [item_length, item_width, 
                          room_x_offset, (room_x_offset-item_width+room_props[1]), 
                          room_y_offset, (room_y_offset-item_length+room_props[0])];
        }
        let item = FurnitureItemStats{
            geometry_properties: gmtr_props.into(),
            r#type: t,
            img: Image::load_from_path(&type_to_img(t)).unwrap()
        };
        real_values_stored.push((item_length_real, item_width_real));
        items_props_list.push(gmtr_props);
        items_list.push(item);

        let new_active_items = Rc::new(VecModel::from(items_list.clone()));
        ui.set_active_items(ModelRc::from(new_active_items));

    }});

    ui.run()
}
