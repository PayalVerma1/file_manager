use std::{
    fs::{self, OpenOptions, create_dir, remove_dir, remove_file, rename},
    io,
};
fn main() {
    println!("Enter file/folder / file_in_folder /remove_folder / remove_file / move");
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim();
    match option {
        "file" => {
            let mut f_name = String::new();
            io::stdin().read_line(&mut f_name).unwrap();
            file(&f_name.trim());
        }

        "folder" => {
            println!("Enter folder name:");
            let mut folder_n = String::new();
            io::stdin().read_line(&mut folder_n).unwrap();
            let folder_n = folder_n.trim();

            folder(folder_n);
        }
        "file_in_folder" => {
            println!("Enter folder name:");
            let mut folder_n = String::new();
            io::stdin().read_line(&mut folder_n).unwrap();
            let folder_n = folder_n.trim();

            folder(folder_n);
            println!("Enter the file name:");
            let mut file_n = String::new();
            io::stdin().read_line(&mut file_n).unwrap();
            let file_n = file_n.trim();

            create_file_in_folder(folder_n, file_n);
        }
        "remove_folder" => {
            println!("Enter the file name");
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            remove_folder(&name.trim());
        }
        "remove_file" => {
            println!("Enter the file name");
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            delete_file(&name.trim());
        }
        "move" => {
            println!("Enter the currrent file path");
            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path).unwrap();
            println!("Enter the folder path");
            let mut folder_path = String::new();
            io::stdin().read_line(&mut folder_path).unwrap();
            let new_path = format!("{}/{}", folder_path, file_path);
            match fs::rename(file_path, &new_path) {
                Ok(_) => println!("Moved file to {}", new_path),
                Err(e) => println!("Error moving file: {}", e),
            }
        }
        _ => {
            println!("...")
        }
    }
}
fn file(e: &str) {
    let filename = format!("{}.txt", e);

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)
        .expect("can't open file");

    println!("File created: {}", filename);
}

fn folder(name: &str) {
    match create_dir(name) {
        Ok(_) => println!("Folder created: {}", name),
        Err(_) => println!("Folder already exists"),
    }
}

fn create_file_in_folder(folder: &str, file: &str) {
    let path = format!("{}/{}.txt", folder, file);
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("can't create file in folder");
    println!("File created inside folder: {}", path);
}
fn remove_folder(name: &str) {
    match remove_dir(name) {
        Ok(_) => println!("Removed {}", name),
        Err(_) => println!("Can't be removed"),
    }
}
fn delete_file(name: &str) {
    match remove_file(name) {
        Ok(_) => println!("Removed {}", name),
        Err(_) => println!("Can't be removed"),
    }
}
 
