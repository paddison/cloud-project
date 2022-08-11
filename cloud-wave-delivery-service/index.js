const AWS = require('aws-sdk');

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
    console.log('Received event:', JSON.stringify(event, null, 2));

    let body;
    let isBase64Encoded = false;
    let statusCode = '200';
    //todo: set depending on response
    let headers = {
        'Content-Type': 'application/json',
    };
    const request_id = event.queryStringParameters.request_id;
    const file_id = event.queryStringParameters.file_id;
    
    //s3 bucket
   // Create S3 service object
    const s3 = new AWS.S3({apiVersion: '2006-03-01'});
    
    // Create the parameters for getting an object
    const objParams = {
      Bucket : process.env.BUCKET_NAME,
      Key: file_id + ".wav"
    };
    
    try {
        switch (event.httpMethod) {
            case 'GET':
                
                var params = {
                  TableName : process.env.TABLE_NAME,
                  Key: {
                    id: file_id
                  },
                  ProjectionExpression: 'request_id, is_downloaded'
                };

                const data = await dynamo.get(params).promise();
                console.log("data from db: " + JSON.stringify(data))
                
                if (data.Item.request_id === request_id) {
                    if (data.Item.is_downloaded) {
                        const errMessage = "Corresponding file to request_id already downloaded. Request not valid."
                        console.log(errMessage)
                        throw new Error(errMessage);
                
                    } else {
                        let ready = false;
                        let file;
                        try {
                            const s3Object = await (s3.getObject(objParams).promise());
                            file = Buffer.from(s3Object.Body);
                            ready = true;
                            console.log(file.toString('base64'))
                        } catch (err) {
                            console.log(err);
                        }

                        if (ready) {
                             //application/octet-stream
                             
                            headers = {
                                'Content-Type': 'text/plain',
                            };
                            isBase64Encoded = true;
                            body = file.toString('base64');
                        } else {
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
    } finally {
       // body = JSON.stringify(body);
    }

    return {
        isBase64Encoded,
        statusCode,
        body,
        headers,
    };
};
