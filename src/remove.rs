pub fn console_log(string: &String) -> String {

    let mut new_string = String::new();

    let lines: Vec<&str> = string.split("\n").collect();

    let mut counter = 0;

    for line in lines {

        if line.contains("console.log") {
            println!("{}", line);
            counter += 1; 
            continue
        } else {
            new_string.push_str(line);
            new_string.push_str("\n");
        }
    }

    println!("{} lines removed", counter);

   new_string   

}




