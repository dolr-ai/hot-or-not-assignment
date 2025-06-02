# Brief

Build a TODO app backend CRUD API using [Axum](https://github.com/tokio-rs/axum). The app should have the following endpoints:

- Add a new todo with the todo text as the input and the created todo ID as the output
- Read an existing todo by id
- Read all todos with pagination
- Update a todo with the updated text and existing id
- Delete a todo by id

# Submittable

- URL to the public source code repository

# Additional Information

- The above code is expected to be performant and scalable and following best practices
- The app must store its state on the filesystem in json format
- The app must not lose the created todos across restarts and crashes
