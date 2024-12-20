#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("FileDescription", "GPT Shell - 命令行AI助手")
        .set("ProductName", "GPT Shell")
        .set("OriginalFilename", "gpt.exe")
        .set("LegalCopyright", "Copyright © 2024")
        .set("CompanyName", "Your Company Name");
    
    if let Err(e) = res.compile() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(not(windows))]
fn main() {} 