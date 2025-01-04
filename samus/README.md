# SAMUS

A simple Redis-like volatile memory store behind a socket server.

### How to use
Currently, the Samus socket server responds to the following command syntax:

For getting a key:
`GET <key_name>`

For setting a key:
`SET <key_name> <key_value> <key_ttl>`

Note: key values are represented internally as strings.
Note: key ttl is given as whole seconds, and is not currently implemented (it will do nothing).

For deleting a key:
`DELETE <key_name>`

Be sure to add a newline character (`\n`) at the end of your queryies.
You can send as many queries as you want as one message (connection) to the socket as a newline seperated list. Make sure to include a trailing newline at the end of the list.

The server will respond with a response to your query. Look for the string `__TERM__` in your message. When it appears, the socket server has closed its connection to your client. You will need a new connection to send a new query.

### What's next?
- Message TTL implementation
- Multi-threaded server capability
- Refine ergonomics around network protocol
