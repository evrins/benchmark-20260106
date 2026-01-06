const express = require('express');
const PORT = 8080;
  // This code runs in all worker processes
const app = express();

app.get('/', (req, res) => {
  res.send({code: 0, msg: 'OK', data: {}});
});


app.listen(PORT, () => {
  console.log(`Worker ${process.pid} started and listening on port ${PORT}`);
});
