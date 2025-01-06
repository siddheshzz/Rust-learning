fn main() {
    // simple_logger::SimpleLogger::new().init().unwrap();

    // log::trace!("This is a simple sample trace");
    // log::debug!("This is a sample debug.");
    // log::info!("This is a sample info.");
    // log::warn!("This is a sample warn.");
    // log::error!("This is a sample error.");

    
    // simple_logger::init().unwrap();

    // log::trace!("This is a simple sample trace");
    // log::debug!("This is a sample debug.");
    // log::info!("This is a sample info.");
    // log::warn!("This is a sample warn.");
    // log::error!("This is a sample error.");

    //Simple logger level
    simple_logger::init_with_level(log::Level::Info).unwrap();

    log::info!("This is a sample info.");
    log::trace!("This is a sample trace.");
    log::debug!("This is a sample debug.");
    
    log::warn!("This is a sample warn.");
    log::error!("This is a sample error.");


    
}
