// Lambdas

# # # # # 
# wave_delivery_service
# # # # #
//the zipped function source code
data "archive_file" "wave_delivery_service_zip" {
  type        = "zip"
  source_dir  = "${path.cwd}/cloud-wave-delivery-service" 
  output_path = "builds/wave_delivery_service.zip" 
}
//the lambda service role
resource "aws_iam_role" "wave_delivery_service_role" {
  name = "${var.wave_delivery_service_name}_role" 
  assume_role_policy = jsonencode({
  Version = "2012-10-17"
  Statement = [
    {
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Principal = {
        Service = "lambda.amazonaws.com"
      }
    },
  ]
})
}
//the function and its config
resource "aws_lambda_function" "wave_delivery_service" {
  filename      = data.archive_file.wave_delivery_service_zip.output_path
  function_name = var.wave_delivery_service_name 
  role          = aws_iam_role.wave_delivery_service_role.arn
  handler       = "index.handler"
  runtime       = "nodejs12.x"

  environment {
    variables = {"TABLE_NAME"="${var.TABLE_NAME}" 
    "BUCKET_NAME"="${var.BUCKET_NAME}"}
  }

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
resource "aws_iam_role_policy_attachment" "wave_delivery_service_right_basic" {
  role       = aws_iam_role.wave_delivery_service_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

// // //
// API Gateway
resource "aws_api_gateway_rest_api" "wave_delivery_service_API" {
  name = "${var.wave_delivery_service_name}_API"

  endpoint_configuration {
    types = ["REGIONAL"]
  }
}
//the API resource
resource "aws_api_gateway_resource" "wave_delivery_service_API_resource" {
  rest_api_id = aws_api_gateway_rest_api.wave_delivery_service_API.id
  parent_id   = aws_api_gateway_rest_api.wave_delivery_service_API.root_resource_id
  path_part   = "wave-delivery-service" // replace
}
//the API method
resource "aws_api_gateway_method" "wave_delivery_service_API_method" {
  rest_api_id   = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id   = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method   = "GET"
  authorization = "NONE"
  request_parameters = {"method.request.querystring.file_id" = true
                        "method.request.querystring.request_id" = true
                        "method.request.querystring.offset_num" = true}
}
//the API integration
resource "aws_api_gateway_integration" "wave_delivery_service_API_integration" {
  rest_api_id   = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id   = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method   = aws_api_gateway_method.wave_delivery_service_API_method.http_method
  integration_http_method = "POST"
  type          = "AWS" //???
  uri           = aws_lambda_function.wave_delivery_service.invoke_arn
  # Transforms the incoming XML request to JSON
  request_templates = {
    "application/json" = <<EOF
{
    "queryStringParameters": {
         "file_id": "$input.params('file_id')",
         "request_id": "$input.params('request_id')",
         "offset_num": "$input.params('offset_num')"
         },
    "httpMethod": "GET"
}
EOF
  }

  depends_on = [
    aws_api_gateway_method.wave_delivery_service_API_method
  ]
}
//the API integration response
resource "aws_api_gateway_integration_response" "wave_delivery_service_API_integration_response" {
  rest_api_id = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method = aws_api_gateway_method.wave_delivery_service_API_method.http_method
  status_code = aws_api_gateway_method_response.wave_delivery_service_API_method_response.status_code
  # allow all origins
  response_parameters = { "method.response.header.Access-Control-Allow-Origin" = "'*'" }

  depends_on = [
    aws_api_gateway_integration.wave_delivery_service_API_integration
  ]
}
//the API method response //terraform import aws_api_gateway_method_response.wave_delivery_service_API_method_response fucavbi4qg/0avoxq/GET/200
resource "aws_api_gateway_method_response" "wave_delivery_service_API_method_response" {
  rest_api_id = aws_api_gateway_rest_api.wave_delivery_service_API.id
  resource_id = aws_api_gateway_resource.wave_delivery_service_API_resource.id
  http_method = aws_api_gateway_method.wave_delivery_service_API_method.http_method
  status_code = "200"

  response_parameters = { "method.response.header.Access-Control-Allow-Origin" = true }
  response_models = {
    "application/json" = "Empty"
  }

  depends_on = [
    aws_api_gateway_method.wave_delivery_service_API_method
  ]
}
//the permission to invoke the lambda
resource "aws_lambda_permission" "wave_delivery_permission" {
  statement_id  = "AllowWaveDeliveryAPIInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.wave_delivery_service.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn = "${aws_api_gateway_rest_api.wave_delivery_service_API.execution_arn}/*/GET/wave-delivery-service"
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

  depends_on = [
    aws_api_gateway_integration.wave_delivery_service_API_integration
  ]

}
//the gateway stage
resource "aws_api_gateway_stage" "demo" {
  deployment_id = aws_api_gateway_deployment.wave_delivery_service.id
  rest_api_id   = aws_api_gateway_rest_api.wave_delivery_service_API.id
  stage_name    = "demo" // replace with variable
}
//enabling CORS for the gateway
module "api-gateway-enable-cors" {
  source  = "squidfunk/api-gateway-enable-cors/aws"
  version = "0.3.3"
  api_id          = aws_api_gateway_rest_api.wave_delivery_service_API.id
  api_resource_id = aws_api_gateway_resource.wave_delivery_service_API_resource.id
}
