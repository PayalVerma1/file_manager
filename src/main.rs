use std::{
    fs::{self, OpenOptions, create_dir},
    io,
};
fn main() {
    println!("Enter file/folder");
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
        "file_in_folder"=>{
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

fn create_file_in_folder(folder:&str, file:&str){
    let path=format!("{}/{}.txt",folder,file);
    OpenOptions::new().create(true).append(true).open(&path).expect("can't create file in folder");
    println!("File created inside folder: {}",path);


}