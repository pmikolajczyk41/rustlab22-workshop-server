/// Calling an external service from your test suite is not ideal, because:
/// - If the service is down, your test suite fails.
/// - You could incur in rate limits.
/// - Your tests require an internet connection to pass.
///
/// To avoid calling directly https://swapi.dev/,
/// run the swapi docker image locally.
///
/// ```sh
/// # Change directory to where you cloned https://github.com/MarcoIeni/swapi
/// cd swapi
///
/// # Build the docker image
/// docker image build -t swapi .
///
/// # Run the docker image.
/// # The port 9992 must be free. You can use a different port if 9992 isn't free.
/// docker run -p 9992:8000 -it swapi
///
/// # You should see info about Luke Skywalker
/// curl http://127.0.0.1:9992/api/people/1/
/// ```
///
/// Now you can also see Swapi docs by opening
/// `http://127.0.0.1:9992/` in your browser.
#[test]
fn run_the_swapi_docker_image() {
    let is_swapi_running = true;

    assert!(is_swapi_running);
}
