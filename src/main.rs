extern crate csv;
// extern crate rustc_serialize;

use csv::{Reader, Writer, WriterBuilder};
// use str::env;
use std::io;
use std::fmt::Display;
use std::*;
use std::fs;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::prelude::*;
use std::os::unix;
use std::fs::DirEntry;
use std::path::Path;
use std::fs::read_dir;
use std::ptr::write;


fn main() {

    for entry_res in read_dir("users_images").unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let mut file_name = file_name_buf.into_vec().unra;
        println!("{:?}", file_name);

        // for line in file_name_buf (){
        //     let header = line;
        //     let mut file = OpenOptions::new()
        //         .write(true)
        //         .append(true)
        //         .open("client_list.csv")
        //         .unwrap();
        //     let mut wtr = csv::Writer::from_writer(file);
            // Writer::write_field(&mut wtr, file_name_buf);
        }




        // let dollar_films = vec![
        //     ("A Fistful of Dollars", "Rojo", 1964u),
        //     ("For a Few Dollars More", "El Indio", 1965u),
        //     ("The Good, the Bad and the Ugly", "Tuco", 1966u),
        // ];
        // let path = Path::new("client_list.csv");
        // let mut writer_csv = Writer::from_path("client_list.csv");
        // for row in file_name.into_ {
        //     writer_csv.e(row).ok().expect("CSV writer error");
        // }
        // // let mut mylist_csv = read_to_string("client_list.csv")
        // //     .expect("somthing went wrooong");
        //
        // // let mut new_list = mylist_csv.to_string().push_str(file_name);
        // let mut mylist = Vec::new();
        // mylist.push(file_name_buf);
        // // if !file_name.starts_with(".") &&
        // //     entry.file_type().unwrap().is_dir()
        // // {
        //
        //     println!("File {:?} has full path {:?}",
        //              file_name_buf, entry.path());
        // }
    }

    // let mut entries = fs::read_dir("users_images")?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;
    //
    // // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // // ordering is required the entries should be explicitly sorted.
    //
    // println!("{:?}",entries.sort());
    //
    // // The entries have now been sorted by their path.
    //
    // Ok(())
    //
    // // let image_path = "users_images";
    // // println!("{:?} ", image_path);
    // // let mylist =
    // // // let images = vec![];
    // // // let classNames = vec![];
    // // // let myList = IMAGE_PATH.unwrap().path().display();
    // // println!("{:?} ",mylist);
    // // // println!("{:?} ", classNames);
