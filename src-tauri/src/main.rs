// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::result::Result::Err;
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
              window: tauri::Window) -> Result<(), String> {
    // set filepath
    let _path_file_input = Path::new(&file_input);
    let _path_dir_input = Path::new(&dir_input);
    let _path_dir_output = Path::new(&dir_output);
    let _path_file_output = _path_dir_output.join(&file_output);

    // Read spreadsheet (using calamine)
    let mut _book_in: Xlsx<_> = open_workbook(&_path_file_input).unwrap();
    let sheet_names = _book_in.sheet_names();
    // Write spreadsheet (using rust_xlsxwriter)
    let mut _book_out = Workbook::new();
    let worksheet = _book_out.add_worksheet();

    if let Some(Ok(range)) = _book_in.worksheet_range(&sheet_names[0].to_string()) {
        // get numbers of rows and columns
        // _nr: row number of data (exclude header)
        let _nr = range.get_size().0 - 1;
        let _nc = range.get_size().1;

        // get column number of "filename"
        let nc_filename = get_filename_column_number(&range, _nc, &window);
        if nc_filename.0 == false {
            return Err("Column named filename is missing".to_string());
        }

        // get shuffled ID
        let id = rnd_index(_nr);

        // write headers
        // headers in input file
        for j in 0.._nc {
            let _value = range.get_value((0, j as u32)).unwrap();
            set_value_cell(worksheet, 0, (j + 1) as u16, _value);
        }
        // headers for ID / new filename
        let _ = worksheet.write_string(0, 0, "ID");
        let _ = worksheet.write_string(0, (_nc + 1) as u16, "new_filename");

        // write contents (shuffled order) / copy files
        for i in 0.._nr {
            // write contents in input file
            for j in 0.._nc {
                let _value = range.get_value(((i + 1) as u32, j as u32)).unwrap();
                set_value_cell(worksheet, id[i] as u32, (j + 1) as u16, _value);
            }
            // write "ID" column
            let _ = worksheet.write_number(id[i] as u32, 0, (i + 1) as u32);

            // "new_filename" column
            // set new filename
            let _file = range.get_value(((i + 1) as u32, nc_filename.1 as u32))
                .unwrap().to_string();
            let file_path = Path::new(&_file);
            let _new_file = format!("shuffled_{:>03}.{}",
                                    id[i],
                                    file_path.extension()
                                    .unwrap()
                                    .to_string_lossy());

            // set filepath
            let _path_from = &_path_dir_input.join(&_file);
            let _path_to = &_path_dir_output.join(&_new_file);

            if Path::is_file(&_path_from) {
                // when file is present
                // write new filename to cell
                let _ = worksheet.write_string(id[i] as u32, (_nc + 1) as u16,
                                               &_new_file);
                // copy files
                let _ = std::fs::copy(&_path_from, &_path_to);
            } else {
                // show dialog when file is missing
                message(Some(&window), "Warning",
                        format!("{} is missing!", &_file));
            }

            // autowidth worksheet
            worksheet.autofit();
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
// args:
//    range
//    col: max column number
// return:
//    (bool, usize)
fn get_filename_column_number(range: &Range<DataType>,
                              col: usize,
                              _window: &tauri::Window) -> (bool, usize) {
    // show dialog
    for j in 0..col {
        let _value = range.get_value((0, j as u32)).unwrap();
        if _value == "filename" {
            return (true, j as usize);
        }
    }

    // show dialog
    message(Some(&_window), "Error",
            "Column named 'filename' is missing!");
    return (false, 0 as usize);
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

