use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    // 1. Create a server in the background for us to send requests to
    // 2. create a new client we can use to send our request.
    let address = spawn_app();  
    let client = reqwest::Client::new();

    // Act
    // Using the client, set a GET request
    // Send the request and wait for the response
    // store the response in `response`
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    // Check that the response was successful, status == OK 200
    // Check that the response body was empty
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // Get the new random port
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // return application address
    format!("http://127.0.0.1:{}", port)
}
