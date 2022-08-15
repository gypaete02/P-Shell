use std::path::{PathBuf, MAIN_SEPARATOR};
use std::fs;


// TODO
pub fn complete(string: &mut String) {

    if PathBuf::from(string.as_str()).exists() {

        // TODO: list all dir entries, select them with tab / backtab

       // let entries = fs::read_dir(string).unwrap();

    } else {

        // TODO: list all dir entries that match the unfinished path. tab and backtab to select.

        //let parts = string.split(MAIN_SEPARATOR);
    }

}

//  elias/Documents/tes
//
//
//
//