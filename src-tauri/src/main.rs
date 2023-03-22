// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;
use calamine::{Reader, Xlsx, open_workbook, DataType, Range};
use rust_xlsxwriter::{Workbook, Worksheet};
use tauri::api::dialog::message;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            do_shuffle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn do_shuffle(file_input: &str,
              dir_input: &str,
              file_output: &str,
              dir_output: &str,
              window: tauri::Window ) -> Result<(), String> {
    // set filepath
    let _path_file_input = Path::new(&file_input);
    let _path_dir_input = Path::new(&dir_input);
    let _path_dir_output = Path::new(&dir_output);
    let _path_file_output = _path_dir_output.join(&file_output);

    // Read spreadsheet (using calamine)
    let mut _book_in: Xlsx<_> = open_workbook(&_path_file_input).unwrap();
    // Write spreadsheet (using rust_xlsxwriter)
    let mut _book_out = Workbook::new();
    let worksheet = _book_out.add_worksheet();

    if let Some(Ok(range)) = _book_in.worksheet_range("Sheet1") {
        // get numbers of rows and columns
        // _nr: header + rows of data
        let _nr = range.get_size().0;
        let _nc = range.get_size().1;

        // get column number of "filename"
        let nc_filename = get_filename_column_number(&range, _nc);

        // shuffle ID
        let id = rnd_index(_nr - 1);

        // write headers of original data
        for j in 0.._nc {
            let _value = range.get_value((0, j as u32)).unwrap();
            set_value_cell(worksheet, 0, (j + 1) as u16, _value);
        }
        // write headers for ID / new filename
        let _ = worksheet.write_string(0, 0, "ID");
        let _ = worksheet.write_string(0, (_nc + 1) as u16, "new_filename");

        // write contents (shuffled order)
        for i in 1.._nr {
            // contents
            for j in 0.._nc {
                let _value = range.get_value((i as u32, j as u32)).unwrap();
                set_value_cell(worksheet, id[i - 1] as u32, (j + 1) as u16, _value);
            }

            // ID
            let _ = worksheet.write_number(id[i - 1] as u32, 0, i as u32);

            // set new filename
            let _file = range.get_value((i as u32, nc_filename as u32))
                .unwrap().to_string();
            let file_path = Path::new(&_file);
            let _new_file = format!("shuffled_{:>03}.{}",
                                    id[i - 1],
                                    file_path.extension()
                                    .unwrap()
                                    .to_string_lossy());
            // write new filename into cells
            let _ = worksheet.write_string(id[i - 1] as u32, (_nc + 1) as u16,
                                           &_new_file);

            // autowidth
            worksheet.autofit();

            // copy files
            copy_file(&_file, &_path_dir_input,
                      &_new_file, &_path_dir_output,
                      &window);
        }
        // show dialog
        message(Some(&window), "Message", "Shuffle is completed.");

        // save xlsx file
        let _ = _book_out.save(&_path_file_output);
        Ok(())
    } else {
        Err("Can not read/write worksheet.".to_string())
    }
}


// shuffle index
fn rnd_index(n: usize) -> Vec<usize> {
    let mut rng = thread_rng();
    let mut id = vec![0; n as usize];
    for i in 0..n {
        id[i] = i + 1;
    }

    id.shuffle(&mut rng);
    id
}


// get column number of "filename"
fn get_filename_column_number(range: &Range<DataType>, col: usize) -> usize {
    let mut nc_filename: usize = 0;
    for j in 0..col {
        let _value = range.get_value((0, j as u32)).unwrap();
        if _value == "filename" {
            nc_filename = j;
            break;
        }
    }
    nc_filename
}


// write value to cell (using rust_xlsxwriter)
// worksheet: Worksheet
// row: u32
// col: u16
// value: value from calamine
//   (example)
//   let value = range.get_value((i as u32, j as u32)).unwrap()
fn set_value_cell(worksheet: &mut Worksheet,
                  row: u32, col: u16,
                  value: &DataType) {
    let _ = match value {
        DataType::Float(value) =>
            worksheet.write_number(row, col, *value),
        DataType::String(value) =>
            worksheet.write_string(row, col, value),
        _ => worksheet.write_string(row, col, ""),
    };
}

// copy files
fn copy_file(_file: &String,
             _path_dir_input: &Path,
             _new_file: &String,
             _path_dir_output: &Path,
             _window: &tauri::Window){
    let _path_from = &_path_dir_input.join(&_file);
    let _path_to = &_path_dir_output.join(&_new_file);

    if Path::is_file(&_path_from) {
        let _ = std::fs::copy(&_path_from, &_path_to);
    } else {
        // show dialog
        message(Some(&_window), "Warning",
                format!("{} is missing!", &_file));
    }
}
