const server = Bun.serve({
  // `routes` requires Bun v1.2.3+
  routes: {
    "/":  Response.json({msg: "OK", code: 0, data: {}}),
  },

  port: 8080
});

console.log(`Server running at ${server.url}`);


