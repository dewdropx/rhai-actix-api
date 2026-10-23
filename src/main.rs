use actix_web::{
    HttpServer, // creates, runs http server
    get, // macro, define get request handlers
    App, // represents web application and its routes
    web::Path, // extract values from url path
    Responder, // trait for types that can be returned as http responses
};

use rhai::Engine; // to execute .rhai files

// get endpoint that accepts 2 numbers in the url for multiplying
#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder {

    // extract the numbers
    let (num1, num2) = path.into_inner();

    // new rhai scripting engine, register rust functions that rhai can call to retrieve the input numbers, `move` transfers the ownership of each number
    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    // calculate and get result
    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();

    format!("{result}") // format to string
}

// get endpoint that accepts 2 numbers in the url for adding
#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {

    // extract the numbers
    let (num1, num2) = path.into_inner();

    // new rhai scripting engine, register rust functions that rhai can call to retrieve the input numbers, `move` transfers the ownership of each number
    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    // calculate the result
    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    format!("{result}") // format to string
}

// set up asynchronous run time using the actix macro
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // create new http server and config its application routes
    HttpServer::new(|| {
        App::new()
            .service(multiply)
            .service(add)
    })
        // bind the server to port: 8080
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run() // start server
        .await
}