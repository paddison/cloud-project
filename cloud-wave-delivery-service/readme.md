# Description

This Lambda is responsible for delivering the requested file to the client. If the file exceeds a certain maximum of a payload size, just a part will be sent. The frontend is in charge of keeping the state.