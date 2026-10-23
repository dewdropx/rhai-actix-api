# Arithmetic HTTP API in Rust Using Actix Web and Rhai

A simple Rust web API that performs addition and multiplication using **Actix Web** for HTTP routing and **Rhai** for executing external scripts.

## Overview

This project demonstrates how to build an HTTP API in Rust and integrate an embedded scripting engine to separate arithmetic logic from the web server implementation.

The API accepts two integers through URL path parameters, passes them to the Rhai engine, executes the corresponding `.rhai` script, and returns the result as an HTTP response.

## Features

- **Actix Web:** Handles HTTP requests and defines API endpoints.
- **Rhai Scripting:** Executes arithmetic operations from external script files.
- **URL Path Parameters:** Accepts two integers as input.
- **Separate Logic Files:** Keeps addition and multiplication logic in individual scripts.
- **Asynchronous Server:** Uses Actix Web's asynchronous runtime.

## Technologies Used

- Rust (Edition 2024)
- Actix Web
- Rhai
- Cargo

## API Endpoints

| Method | Endpoint | Description |
|---|---|---|
| GET | `/add/{num1}/{num2}` | Adds two integers |
| GET | `/multiply/{num1}/{num2}` | Multiplies two integers |

### Example Requests

**Addition**

```text
http://127.0.0.1:8080/add/10/5
```

Response:

```text
15
```

**Multiplication**

```text
http://127.0.0.1:8080/multiply/6/7
```

Response:

```text
42
```

## Rhai Scripts

### `add.rhai`

```javascript
fn add(num1, num2) {
    return num1 + num2;
}

let num1 = num1();
let num2 = num2();

add(num1, num2)
```

### `multiply.rhai`

```javascript
fn multiply(num1, num2) {
    return num1 * num2;
}

let num1 = num1();
let num2 = num2();

multiply(num1, num2)
```

The Rust application registers `num1()` and `num2()` as Rhai-callable functions. Each script retrieves these values and performs its respective arithmetic operation.

## How to Run

### Prerequisites

- Install Rust and Cargo using [rustup](https://rustup.rs/).
- Ensure that the required dependencies are listed in `Cargo.toml`.

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/dewdropx/rhai-actix-api.git
   ```

2. Navigate to the project directory:

   ```bash
   cd rhai-actix-api
   ```

3. Run the application:

   ```bash
   cargo run
   ```

4. Open the example endpoint URLs in your browser.

The server runs at `http://127.0.0.1:8080`.
