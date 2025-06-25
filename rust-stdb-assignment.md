# Brief

Build a TODO App Backend CRUD API using [SpaceTimeDB](https://spacetimedb.com/). The app should have the following endpoints:

- Add a new todo with the todo text as the input and the created todo ID as the output
- Read an existing todo by id
- Read all todos with pagination
- Update a todo with the updated text and existing id
- Delete a todo by id

Ensure that each user has a different state

Also create a CLI client which uses async rust to perform at least three operations: Add, Read by ID, Delete by ID.  
The SpaceTime SDK uses a callback mechanism as opposed to async/await. You are expected to bridge them.

Submittable

- URL to the public source code repository
- Link to the hosted module on maincloud ([Example](https://spacetimedb.com/my-cool-module))

Additional Information

- The Backend code is expected to be performant and following best practices
- SpaceTimeDB mainclouud gives you 25 TeV energy for free
