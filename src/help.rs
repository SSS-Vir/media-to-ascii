pub fn usage_message() {
    println!("usage: hello FILE_PATH [OPTIONS]");
    println!("Options for .gif files:");
    println!("  --fps=FPS\tGif's FPS (default 15)");
    println!("Options for jpg, jpeg, png:");
    println!("  --nosave\tDont save ascii to a file(default false)");
    println!("  --output FILE\tSaves ascii to a file(default creates file in working directory)");
    println!("  --size=WIDTHxHEIGHT\t Changes size of picture");
    println!("  --noprint\tDon't print to console(default false)");
    println!("General options:");
    println!("  --colored\tMakes ascii colored");
}
