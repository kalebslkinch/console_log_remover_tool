pub fn console_log(string: &String) -> String {

    let mut new_string = String::new();

    let lines: Vec<&str> = string.split("\n").collect();

    // check lines for console.log
    for line in lines {

        if line.contains("console.log") {
            // remove entire line 
            println!("{}", line);
            continue
        } else {
            // add line to new_string
            new_string.push_str(line);
            new_string.push_str("\n");
        }
    }




   new_string   

}




