/* Okay, so instead of making macros the default way, I'm making it `optional` way */

// those who find macros interfacting simpler, can use it, otherwise the primary mode of routing will be functions.

/* there will be various variant of handling function, 
so routing will actually simplify that,
 by automatically infering what function to use based on number of inputs */

#[macro_export]
macro_rules! route {
    ($path:expr, $handler:expr) => { 
        $handler;
     };
}