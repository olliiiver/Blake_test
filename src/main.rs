fn main() {
       // The message we want to hash
       let message = "Hello, World!";

       // Compute the BLAKE3 hash of the message
       let hash = blake3::hash(message.as_bytes());
}
