// Lambdas

# # # # # 
# main lambda
# # # # #
//the lambda service role
resource "aws_iam_role" "main_lambda_role" {
  name = "main_lambda_role" // replace with variable?
}
//the function and its config
resource "aws_lambda_function" "main_lambda" {
  filename      = "" // replace with path of artifact
  function_name = "main_lambda" // replace with variable
  handler       = "bootstrap"
  role          = aws_iam_role.main_lambda_role.arn
  runtime       = "provided"

  ephemeral_storage {
    size = 512 # Min 512 MB and the Max 10240 MB
  }
}
//assign rights to lambda --> AWSLambdaBasicExecutionRole??
resource "aws_iam_role_policy_attachment" "main_lambda_right" {
  role       = aws_iam_role.main_lambda_role.name
  policy_arn = aws_iam_policy.dynamodb_read_list_everybody_policy.arn
}
resource "aws_iam_role_policy_attachment" "main_lambda_right2" {
  role       = aws_iam_role.main_lambda_role.name
  policy_arn = aws_iam_policy.dynamodb_write_wave_table_policy.arn
}
resource "aws_iam_role_policy_attachment" "main_lambda_right3" {
  role       = aws_iam_role.main_lambda_role.name
  policy_arn = aws_iam_policy.invoke_sine_generator_policy.arn
}

// // //
// API Gateway
resource "aws_api_gateway_rest_api" "main_lambda_API" {
  name = "main_lambda_API" // replace with variable

  endpoint_configuration {
    types = ["REGIONAL"]
  }
}
//the API resource
resource "aws_api_gateway_resource" "main_lambda_API_resource" {
  rest_api_id = aws_api_gateway_rest_api.main_lambda_API.id
  parent_id   = aws_api_gateway_rest_api.main_lambda_API.root_resource_id
  path_part   = "/main_lambda" //replace
}
//the API method
resource "aws_api_gateway_method" "main_lambda_API_method" {
  rest_api_id   = aws_api_gateway_rest_api.main_lambda_API.id
  resource_id   = aws_api_gateway_resource.main_lambda_API_resource.id
  http_method   = "POST"
  authorization = "NONE"
}
//the API integration
resource "aws_api_gateway_integration" "main_lambda_API_integration" {
  rest_api_id   = aws_api_gateway_rest_api.main_lambda_API.id
  resource_id   = aws_api_gateway_resource.main_lambda_API_resource.id
  http_method   = aws_api_gateway_method.main_lambda_API_method.http_method
  type          = "AWS" //???
  uri           = aws_lambda_function.main_lambda.invoke_arn
}
//the API integration response
resource "aws_api_gateway_integration_response" "main_lambda_API_integration_response" {
  rest_api_id = aws_api_gateway_rest_api.main_lambda_API.id
  resource_id = aws_api_gateway_resource.main_lambda_API_resource.id
  http_method = aws_api_gateway_method.main_lambda_API_method.http_method
  status_code = aws_api_gateway_method_response.main_lambda_API_method_response.status_code
  # allow all origins
  response_parameters = { "method.response.header.Access-Control-Allow-Origin" = "*" }
}
//the API method response 
resource "aws_api_gateway_method_response" "main_lambda_API_method_response" {
  rest_api_id = aws_api_gateway_rest_api.main_lambda_API.id
  resource_id = aws_api_gateway_resource.main_lambda_API_resource.id
  http_method = aws_api_gateway_method.main_lambda_API_method.http_method
  status_code = "200"

  response_parameters = { "method.response.header.Access-Control-Allow-Origin" = "*" }
  response_models = {
    "application/json" = "Empty"
  }
}
//the gateway deployment   
resource "aws_api_gateway_deployment" "main_lambda" {
  rest_api_id = aws_api_gateway_rest_api.main_lambda_API.id

  triggers = {
    redeployment = sha1(jsonencode(aws_api_gateway_rest_api.main_lambda_API.body))
  }

  lifecycle {
    create_before_destroy = true
  }
}
//the gateway stage
resource "aws_api_gateway_stage" "demo_main" {
  deployment_id = aws_api_gateway_deployment.main_lambda.id
  rest_api_id   = aws_api_gateway_rest_api.main_lambda_API.id
  stage_name    = "demo" // replace with variable
}