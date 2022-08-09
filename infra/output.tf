
output "wave_delivery_service_url" {
  description = "The url of the wave delivery service API"
  value       = "https://${aws_api_gateway_rest_api.wave_delivery_service_API.id}.execute-api.eu-central-1.amazonaws.com/${aws_api_gateway_stage.demo.stage_name}/${aws_api_gateway_rest_api.wave_delivery_service_API.name}"
}


output "main_lambda_url" {
  description = "The url of the main lambda API"
  value       = "https://${aws_api_gateway_rest_api.main_lambda_API.id}.execute-api.eu-central-1.amazonaws.com/${aws_api_gateway_stage.demo_name.stage_name}/${aws_api_gateway_rest_api.main_lambda_API.name}"
}

# arn:aws:lambda:eu-central-1:976294489850:function:wave_delivery_service