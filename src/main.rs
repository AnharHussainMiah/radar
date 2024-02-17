mod logo;
mod docker;

use std::os::unix::net::UnixStream;
use std::io::{ Read, Write };
use crate::docker::Docker;

const VERSION: &str = "0.1.0";

fn main() {
    logo::draw(&VERSION);
    /* ---------------------------------------------------------------------------------------------
    1. find all running containers
    2. cature their image ID + startup/launch command (filter any labels that do not want 
        to be tracked)
    3. pull latest images
    4. compare difference between running image ID vs pulled image ID
    5. IF the images IDs do not match this means we now a newer image
       we now need to shutdown the existing image and then boot up th new one
    6. send out alerts if any
    7. sleep for X and then goto step 1
    --------------------------------------------------------------------------------------------- */
    if let Ok(docker) = Docker::new() {
        if let Some(x) = docker.call("test") {
            println!("{}", x);
        }

    }
}

fn get_all_running_containers() {
    // todo
}

fn get_all_images() {
    // todo
}

fn pull_latest_images() {
    // todo
}

fn get_image_diff() {
    // todo
}

fn trigger_new_laun() {
    // todo
}

fn trigger_alerts() {
    
}
