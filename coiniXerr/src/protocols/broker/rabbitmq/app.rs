





// we use actors which use channels like mpsc to avoid race conditions to build multithreading jobq apps like rabbitmq
// the client app to write pub/sub codes like twitter tweets broadcaster
// rabbitmq server must be installed on the system 
// ...