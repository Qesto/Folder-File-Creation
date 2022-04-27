use std::io::Write;         // pulling the trait into scope for flush()
use std::io::prelude::*;    // pulling the trait into scope for read()

fn main() {
    let path = String::from("/");
    let exit_or_naw = false;
    
    let res_main = basically_error_checker_for_main(path, exit_or_naw);
    if let Err(e) = res_main
    {
        println!("Error in Main: {}", e);
    }
        
    let res_pause = String::from("System Pause");
    basically_error_checker_for_system_pause(res_pause);
}

fn menu_options(changed_path:String, mut exit_value:bool) -> bool {
    while !exit_value {
        let mut input_option = String::new();
        let mut path = changed_path.to_string();

        print!("\n 1) Create Folder w/ or w/o File + Input\n 2) Create File + Input\n 3) Read File\n 4) Change Path\n 5) Exit\n     ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input_option).unwrap();
        input_option = input_option.trim().to_string();
    
        if input_option == String::from("1")
        {
            let mut folder_name = String::new();
            print!(" Enter Folder Name:\n     ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut folder_name).unwrap();
            folder_name = folder_name.trim().to_string();
    
            let mut file_name = String::new();
            print!(" Enter File Name with desired Extension:\n     ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut file_name).unwrap();
            file_name = file_name.trim().to_string();
    
            let mut file_text = String::new();
            print!(" Enter the text you would like to write into the File (for a new line enter in 3 spaces):\n     ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut file_text).unwrap();
            file_text = file_text.trim().to_string();
            file_text = new_line_or_naw(file_text);
    
            let res_dir = String::from("Dir");
            basically_error_checker_and_folder_file_creation(res_dir, path, folder_name, file_name, file_text);
        }
        else if input_option == String::from("2")
        {
            let mut file_name = String::new();
            print!(" Enter File Name with desired Extension:\n     ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut file_name).unwrap();
            file_name = file_name.trim().to_string();
    
            let mut file_text = String::new();
            print!(" Enter the text you would like to write into the File (for a new line enter in 3 spaces):\n     ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut file_text).unwrap();
            file_text = file_text.trim().to_string();
            file_text = new_line_or_naw(file_text);

            file_write(path, file_name, file_text);
        }
        else if input_option == String::from("3")
        {
            print!(" Enter the Path to the File: \n     ");
            path.clear();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut path).unwrap();
            path = path.trim().to_string();
            path = path_syntax_error_checker(path);
            
            let mut file_name = String::new();
            print!(" Enter the File name with the extension you would like to read:\n     ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut file_name).unwrap();
            file_name = file_name.trim().to_string();
            
            let res_read = String::from("Read");
            basically_error_checker_and_read_file(res_read, path, file_name);
            println!();
        }
        else if input_option == String::from("4")
        {
            print!(" Enter your new Path:\n     ");
            path.clear();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut path).unwrap();
            path = path.trim().to_string();
            path = path_syntax_error_checker(path);

            exit_value = menu_options(path, exit_value);
        }
        else if input_option == String::from("5")
        {
            println!("\n Exiting the Program\n");
            exit_value = true;
            break;
        }
        else
        {
            println!("\n It ain't workin'!\n");
        }
    }
    return exit_value;
}

fn new_line_or_naw(mut text_of_file:String) -> String {
    while text_of_file.find("   ") != None
    {
        text_of_file = text_of_file.replace("   ", "\n");
    }
    return text_of_file;
}

fn create_dir_with_file(name_of_path:String, name_of_folder:String, name_of_file:String, text_of_file:String) -> std::io::Result<()> {
    let folder_path = name_of_path + &name_of_folder + &String::from("/");
    std::fs::create_dir(&folder_path)?;
    let mut file = std::fs::File::create(folder_path + &name_of_file).expect("create failed");
    file.write_all(text_of_file.as_bytes()).expect("write failed");
    println!("\n Write Successful!\n");
    Ok(())
}

fn file_write(name_of_path:String, name_of_file:String, text_of_file:String) {
    let mut file = std::fs::File::create(name_of_path + &name_of_file).expect("create failed");
    file.write_all(text_of_file.as_bytes()).expect("write failed");
    println!("\n Write Successful!\n");
}

fn file_read(name_of_path:String, name_of_file:String) -> std::io::Result<()> {
    let mut file = std::fs::File::open((*name_of_path).to_string() + &name_of_file).expect("open failed");
    let mut line = Vec::new();
    print!("\n Reading File: '{}' in Path: '{}'\n ---------------------------------------------\n", &name_of_file, &name_of_path);
    file.read_to_end(&mut line)?;
    println!("{}", line.iter().map(|&x| x as u8).map(|x| x as char).collect::<String>());
    Ok(())
}

fn basically_system_pause() -> std::io::Result<()> {
    // A way to accomplish a 'system pause' in the cmd.exe file
    print!("\n Press 'Enter' or 'Return' to continue...");
    std::io::stdout().flush()?;             // Manually flushes the buffer
    std::io::stdin().read(&mut [0u8])?;     // Reads the buffer and stores it into a value of 0 as a unsigned 8-bit int
    print!("\n");
    Ok(())
}

fn path_syntax_error_checker(mut name_of_path:String) -> String {
    if name_of_path.find(":") != None
    {
        if name_of_path.chars().nth(0) == Some('/') || name_of_path.chars().nth(0) == Some('\\')
        {
            name_of_path.remove(0);
        }
        if name_of_path.chars().nth(2) != Some('/') && name_of_path.chars().nth(2) != Some('\\')
        {
            let drive_letter = (&name_of_path[0..2]).to_string();
            name_of_path.remove(0);
            name_of_path.remove(0);
            name_of_path = drive_letter + &String::from("/") + &name_of_path;
        }
    }
    else if name_of_path.find(":") == None
    {
        if name_of_path.chars().nth(0) != Some('/') && name_of_path.chars().nth(0) != Some('\\')
        {
            name_of_path = String::from("/") + &name_of_path;
        }
    }
    else
    {
        print!("\n Ay yo! This ain't suppose to occur in fn 'path_syntax_error_checker'\n");
    }
    if name_of_path.chars().rev().nth(0) != Some('/') && name_of_path.chars().rev().nth(0) != Some('\\')
    {
        name_of_path += &String::from("/");
    }

    return name_of_path;
}

fn basically_error_checker_and_folder_file_creation(res_of:String, name_of_path:String, name_of_folder:String, name_of_file:String, text_of_file:String) {
    let res = create_dir_with_file(name_of_path, name_of_folder, name_of_file, text_of_file);
    if let Err(e) = res
    {
        println!("Error in {}: {}", res_of, e);
    }
}

fn basically_error_checker_and_read_file(res_of:String, name_of_path:String, name_of_file:String) {
    let res = file_read(name_of_path, name_of_file);
    if let Err(e) = res
    {
        println!("Error in {}: {}", res_of, e);
    }
}

fn basically_error_checker_for_main(name_of_path:String, exit_value:bool) -> std::io::Result<()> {
    menu_options(name_of_path, exit_value);
    Ok(())
}

fn basically_error_checker_for_system_pause(res_of:String) {
    let res_pause = basically_system_pause();
    if let Err(e) = res_pause
    {
        println!("Error in {}: {}", res_of, e);
    }
}