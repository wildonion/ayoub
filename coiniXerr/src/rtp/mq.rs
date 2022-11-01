




/*





    client
        | 
        -------- rtp mq pub/sub app -------- hoopoe hyper server 
                        |                             |
                        |                             -------- mongodb
                        |
                        producer/publisher actor 
                                |
                                ---tcp socket--> |broker actor exchanges <---route--> queue buffers like mpsc| 
                                                                    |    
                                                                    ---tcp socket--> consumers/subscribers actors





    https://www.cloudamqp.com/blog/part1-rabbitmq-for-beginners-what-is-rabbitmq.html#exchanges


    • Producer: Application that sends the messages.
    • Consumer: Application that receives the messages.
    • Queue: Buffer that stores messages.
    • Message: Information that is sent from the producer to a consumer through RabbitMQ.
    • Connection: A TCP connection between your application and the RabbitMQ broker.
    • Channel: A virtual connection inside a connection. When publishing or consuming messages from a queue - it's all done over a channel.
    • Exchange: Receives messages from producers and pushes them to queues depending on rules defined by the exchange type. To receive messages, a queue needs to be bound to at least one exchange.
    • Binding: A binding is a link between a queue and an exchange.
    • Routing key: A key that the exchange looks at to decide how to route the message to queues. Think of the routing key like an address for the message.
    • AMQP: Advanced Message Queuing Protocol is the protocol used by RabbitMQ for messaging.




*/



pub mod publishers;
pub mod consumers;