const AWS = require('aws-sdk');

// a dynamo db client
const dynamo = new AWS.DynamoDB.DocumentClient();

/**
 * Demonstrates a simple HTTP endpoint using API Gateway. You have full
 * access to the request and response payload, including headers and
 * status code.
 *
 * To scan a DynamoDB table, make a GET request with the TableName as a
 * query string parameter. To put, update, or delete an item, make a POST,
 * PUT, or DELETE request respectively, passing in the payload to the
 * DynamoDB API as a JSON body.
 */
exports.handler = async (event, context) => {

    // the payload of the POST HTTP message, directed from the corresponding API gateway
    console.log('Received event:', JSON.stringify(event, null, 2));

    // properties for the response payload
    let body;
    let isBase64Encoded = false;
    let isLast = true;
    let statusCode = '200';
    let headers = {
        'Content-Type': 'application/json',
    };

    // size of the buffer parts
    const byteRange = 4096000;

    // url parameters from the original frontend request
    const request_id = event.queryStringParameters.request_id;
    const file_id = event.queryStringParameters.file_id;
    const offsetNum = parseInt(event.queryStringParameters.offset_num);
    
    //s3 bucket
   // Create S3 service object
    const s3 = new AWS.S3({apiVersion: '2006-03-01'});
    
    // the parameters for getting a wave file
    let objParams = {
      Bucket : process.env.BUCKET_NAME, // set on the system by terraform 
      Key: file_id + ".wav",
    };
    
    // the parameters for getting a byte range of a wave file
    let objParamsBuffer = {
      Bucket : process.env.BUCKET_NAME,
      Key: file_id + ".wav",
      Range: "bytes=0-4096000"
    };
    
    try {
        switch (event.httpMethod) {
            case 'GET':
                
                // parameters for a get request on the wave file table in dynamodb
                var params = {
                  TableName : process.env.TABLE_NAME, // set on the system by terraform
                  Key: {
                    id: file_id
                  },
                  ProjectionExpression: 'request_id, is_downloaded'
                };

                console.log("offset_num: " + offsetNum);

                // get some of the meta data of the requested wave file object
                const data = await dynamo.get(params).promise();
                console.log("data from db: " + JSON.stringify(data))
                
                // check if request_id is valid
                if (data.Item.request_id === request_id) {

                    // check if file was already downloaded
                    if (data.Item.is_downloaded) {
                        const errMessage = "Corresponding file to request_id already downloaded. Request not valid."
                        console.log(errMessage)
                        throw new Error(errMessage);
                
                    } else {
                        let ready = false;
                        let file;
                        
                        // check if file is already finished processing, goes to catch block if file not found in s3 bucket
                        try {

                            // load meta data from file to check the size
                            const s3ObjMeta = await (s3.headObject(objParams).promise());
                            console.log("meta: " + JSON.stringify(s3ObjMeta));
                            const objSize = s3ObjMeta.ContentLength;
                            
                            // determines if request is the last part of the file
                            if (objSize - offsetNum * byteRange >= byteRange) {
                                isLast = false;
                            }

                            let s3Object;

                            // sends the file in one piece if size allows it
                            if (isLast && offsetNum === 0) {
                                console.log("isLast is true")
                                s3Object = await (s3.getObject(objParams).promise());

                            
                            // sends only a part of the file by specifing a certain range of bytes
                            } else {
                                const offset = offsetNum * byteRange; 
                               
                                if (isLast) {
                                    console.log("isLast is true")
                                    objParamsBuffer.Range = "bytes=" + offset + "-" + objSize
                                } else {
                                    console.log("isLast is false"); 
                                    objParamsBuffer.Range = "bytes=" + offset + "-" + (offset+byteRange-1)
                                }
                            
                                console.log("objParamsBuffer: " + JSON.stringify(objParamsBuffer))
                            
                                s3Object = await (s3.getObject(objParamsBuffer).promise());
                            }

                            if (isLast) {
                                 
                                // parameters for an update on the wave file in dynamodb
                                var params = {
                                    TableName : process.env.TABLE_NAME, // set on the system by terraform
                                    Key: {
                                         id: file_id
                                    },
                                    UpdateExpression: `set is_downloaded = :is_downloaded`,
                                    ExpressionAttributeValues: {
                                      ":is_downloaded": true
                                    },
                                };
  
                                // sets the is_downloaded field in dynmodb to true
                                const dataDownload = await dynamo.update(params).promise();
                            }
                            
                            file = Buffer.from(s3Object.Body);
                            ready = true;

                        } catch (err) {
                            console.log(err);
                        }

                        if (ready) {
                            // wraps the file (part) as base64 string
                            isBase64Encoded = true;
                            body = {status: "ready", file: file.toString('base64')};
                        } else {
                            // tells the frontend to wait and ask again
                            body = {status: "in_progress"};
                        }
                    }
                } else {
                    const errMessage = "Different client than the one initiating the request. Request not valid."
                    console.log(errMessage)
                    throw new Error(errMessage);
                }
                break;
            default:
                throw new Error(`Unsupported method "${event.httpMethod}"`);
        }
    } catch (err) {
        statusCode = '400';
        body = err.message;
    } 

    return {
        isBase64Encoded,
        isLast,
        statusCode,
        body,
        headers,
    };
};
