// use dragona::macros;

/* importing handlers */
mod handlers;
use handlers::home;
// use handlers::{home, users, delete, dashboard, settings};

fn main() {
    
    let _ = home();

    // route!("/", home());

    // route!("/users", users); //middleware

    // route!("/delete/<id>", delete(m), true); //true for passing variable from path to function

    // route!("/dashboard", dashboard);

    // route!("/settings", settings)

}