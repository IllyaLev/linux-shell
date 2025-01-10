fn main(){
    println!("cargo:rerun-if-changed=icons/icon.ico");
    winres::WindowsResource::new()
        .set_icon("icons/icon.ico")
        .compile()
        .unwrap();
}