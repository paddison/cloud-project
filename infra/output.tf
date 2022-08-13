
locals {
  wave_delivery_service_url = "https://${aws_api_gateway_rest_api.wave_delivery_service_API.id}.execute-api.eu-central-1.amazonaws.com/${aws_api_gateway_stage.demo.stage_name}/${aws_api_gateway_resource.wave_delivery_service_API_resource.path_part}"
  main_lambda_url = "https://${aws_api_gateway_rest_api.main_lambda_API.id}.execute-api.eu-central-1.amazonaws.com/${aws_api_gateway_stage.demo_main.stage_name}/${aws_api_gateway_resource.main_lambda_API_resource.path_part}" 
} 

output "react_bucket_arn" {
  value = aws_s3_bucket.react-website-bucket.arn
}

resource "local_file" "env_vars_for_frontend_build" {
    content     = <<EOF
#! bin/bash/
export REACT_APP_FIRST_REQ_URL=${local.main_lambda_url}
export REACT_APP_SECOND_REQ_URL=${local.wave_delivery_service_url}

cd cloud_s3_frontend

npm install
npm run-script build
zip ../builds/build.zip build
EOF
    filename = "../builds/set_env_vars_for_frontend_build.sh"
}
