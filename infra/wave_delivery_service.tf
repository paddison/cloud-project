// Lambdas

# # # # # 
# wave_delivery_service
# # # # #
//the zipped function source code
data "archive_file" "wave_delivery_service_zip" {
  type        = "zip"
  source_dir  = "../cloud_wave_delivery_service" // replace with var
  output_path = "../builds/wave_delivery_service.zip" // replace with var
}
//the lambda service role
resource "aws_iam_role" "wave_delivery_service_role" {
  name = "wave_delivery_service_role" // replace with variable?
}
//the function and its config
resource "aws_lambda_function" "wave_delivery_service" {
  filename      = archive_file.wave_delivery_service_zip.output_path
  function_name = "wave_delivery_service" // replace with variable
  role          = aws_iam_role.wave_delivery_service_role.arn
  handler       = "index.handler"
  runtime       = "nodejs12.x"

  ephemeral_storage {
    size = 512 # Min 512 MB and the Max 10240 MB
  }

  depends_on = [
    data.archive_file.wave_delivery_service_zip
  ]
}
//assign rights to lambda
resource "aws_iam_role_policy_attachment" "wave_delivery_service_right" {
  role       = aws_iam_role.wave_delivery_service_role.name
  policy_arn = aws_iam_policy.get_wave_files_bucket_policy.arn
}
resource "aws_iam_role_policy_attachment" "wave_delivery_service_right2" {
  role       = aws_iam_role.wave_delivery_service_role.name
  policy_arn = aws_iam_policy.read_and_update_wave_files_db_policy.arn
}

// // //
// API Gateway
resource "aws_api_gateway_rest_api" "wave_delivery_service_API" {
  name = "wave_delivery_service_API" // replace with variable

  endpoint_configuration {
    types = ["REGIONAL"]
  }
}
//the API resource
resource "aws_api_gateway_resource" "wave_delivery_service_API_resource" {
  rest_api_id = aws_api_gateway_rest_api.wave_delivery_service_API.id
  parent_id   = aws_api_gateway_rest_api.wave_delivery_service_API.root_resource_id
  path_part   = "/wave_delivery_service" // replace
}
//the API method
resource "aws_api_gateway_method" "wave_delivery_service_API_method" {
  rest_api_id   = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id   = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method   = "GET"
  authorization = "NONE"
  request_parameters = {"method.request.querystring.file_id" = true
                               "method.request.querystring.request_id" = true}
}
//the API integration
resource "aws_api_gateway_integration" "wave_delivery_service_API_integration" {
  rest_api_id   = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id   = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method   = aws_api_gateway_method.wave_delivery_service_API_method.http_method
  type          = "AWS" //???
  uri           = aws_lambda_function.wave_delivery_service.invoke_arn
  # Transforms the incoming XML request to JSON
  request_templates = {
    "application/json" = <<EOF
{
    "queryStringParameters": {
         "file_id": "$input.params('file_id')",
         "request_id": "$input.params('request_id')"
         },
    "httpMethod": "GET"
}
EOF
  }
}
//the API integration response
resource "aws_api_gateway_integration_response" "wave_delivery_service_API_integration_response" {
  rest_api_id = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method = aws_api_gateway_method.wave_delivery_service_API_method.http_method
  status_code = aws_api_gateway_method_response.wave_delivery_service_API_method_response.status_code
  # allow all origins
  response_parameters = { "method.response.header.Access-Control-Allow-Origin" = "*" }
}
//the API method response //terraform import aws_api_gateway_method_response.wave_delivery_service_API_method_response fucavbi4qg/0avoxq/GET/200
resource "aws_api_gateway_method_response" "wave_delivery_service_API_method_response" {
  rest_api_id = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method = aws_api_gateway_method.wave_delivery_service_API_method.http_method
  status_code = "200"

  response_parameters = { "method.response.header.Access-Control-Allow-Origin" = "*" }
  response_models = {
    "application/json" = "Empty"
  }
}
//the lambda deployment   
resource "aws_api_gateway_deployment" "wave_delivery_service" {
  rest_api_id = aws_api_gateway_rest_api.wave_delivery_service_API.id

  triggers = {
    redeployment = sha1(jsonencode(aws_api_gateway_rest_api.wave_delivery_service_API.body))
  }

  lifecycle {
    create_before_destroy = true
  }
}
//the gateway stage
resource "aws_api_gateway_stage" "demo" {
  deployment_id = aws_api_gateway_deployment.wave_delivery_service.id
  rest_api_id   = aws_api_gateway_rest_api.wave_delivery_service_API.id
  stage_name    = "demo" // replace with variable
}