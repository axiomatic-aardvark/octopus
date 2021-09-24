# Octopus

![small_octo](https://user-images.githubusercontent.com/32264020/134729476-9c20ac1c-9dbd-4316-bc8e-449aa5ba9897.png)

🐙 **Octopus** is a Kraken API client implementation written in Rust. It comes with three functionalities:
- Fetching server time
- Fetching information about a trade pair
- Getting all open orders for a specified user

**Octopus** follows a traditional async REST API client approach, and is equipped with **BDD testing scenarios**, which can be seen in the `features` folder.

# Quickstart 🚀

1. Clone the repo and run `cargo build` in the project root folder.
2. Create a `.env` file and copy the contents from the provided `.env.example` file.
3. In the .env file, specify your api key, api secret and one-time password.
4. Execute `cargo run`.

You should then see the result of the scan in your terminal window:
![image](https://user-images.githubusercontent.com/32264020/134730635-331873cf-4904-43bb-a049-4ba8519dbfbd.png)

If there's an error on one of the three requests, it'll be reported like this:
![image](https://user-images.githubusercontent.com/32264020/134730850-1f2bdbb2-e46e-43ec-9a63-b97819133716.png)

# Tests 🧪

To run the BDD tests, simply run `cargo test` in the project root, which will print a thorough report of the test run:
![image](https://user-images.githubusercontent.com/32264020/134731077-7a93a7e0-1e3b-4415-9060-19bcd977daa2.png)

# Docker 🐳

1. Build a Docker image of **Octopus** with `docker build . -t octopus:v1` (This will also run the tests to check if everything passes).
2. Run the image with `docker run octopus:v1`.
