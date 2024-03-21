use dragona::template::render_out;

pub fn home() -> Result<String, Box<dyn std::error::Error>> {
    
    render_out("react-src/src/index.tsx");

    Ok(String::from("askdj")) //response
    
}

// pub fn users() -> Result<String, Box<dyn std::error::Error>> {
//     Ok(String::new())
// }

/* pub fn delete(_id: u32) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::new())
}

pub fn dashboard() -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::new())
}

pub fn settings() -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::new())
} */