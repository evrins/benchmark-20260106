// Require the framework and instantiate it
const fastify = require('fastify')({ logger: false })

// Declare a route
fastify.get('/', function handler (request, reply) {
  reply.send({ data: {}, code: 0, msg: 'OK' })
})

// Run the server!
fastify.listen({ port: 8080 }, (err) => {
  if (err) {
    fastify.log.error(err)
    process.exit(1)
  }
})