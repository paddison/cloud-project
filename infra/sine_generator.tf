// Lambdas

# # # # # 
# sine generator
# # # # #
//the lambda service role
resource "aws_iam_role" "sine_generator_role" {
  name = "${var.GENERATOR_LAMBDA}_role"
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
resource "aws_lambda_function" "sine_generator" {
  filename      = var.GENERATOR_LAMBDA_BOOTSTRAP 
  function_name = var.GENERATOR_LAMBDA
  role          = aws_iam_role.sine_generator_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2" 
  timeout       = 30 

  source_code_hash = filebase64sha256(var.GENERATOR_LAMBDA_BOOTSTRAP)

  ephemeral_storage {
    size = 512 # Min 512 MB and the Max 10240 MB
  }
}
//assign rights to lambda
resource "aws_iam_role_policy_attachment" "sine_generator_right" {
  role       = aws_iam_role.sine_generator_role.name
  policy_arn = aws_iam_policy.dynamodb_write_wave_table_policy.arn
}
resource "aws_iam_role_policy_attachment" "sine_generator_right2" {
  role       = aws_iam_role.sine_generator_role.name
  policy_arn = "arn:aws:iam::aws:policy/AWSLambdaExecute"
}
resource "aws_iam_role_policy_attachment" "sine_generator_right_basic" {
  role       = aws_iam_role.sine_generator_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

# # # # # 
# bucket cleaner
# # # # #
//the lambda service role
resource "aws_iam_role" "bucket_cleaner_role" {
  name = "${var.CLEANER_LAMBDA}_role" 
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
resource "aws_lambda_function" "bucket_cleaner" {
  filename      = var.CLEANER_LAMBDA_BOOTSTRAP 
  function_name = var.CLEANER_LAMBDA
  role          = aws_iam_role.bucket_cleaner_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2" 

  source_code_hash = filebase64sha256(var.CLEANER_LAMBDA_BOOTSTRAP)

  ephemeral_storage {
    size = 512 # Min 512 MB and the Max 10240 MB
  }
}
//assign rights to lambda
resource "aws_iam_role_policy_attachment" "bucket_cleaner_right" {
  role       = aws_iam_role.bucket_cleaner_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
}
resource "aws_iam_role_policy_attachment" "bucket_cleaner_right2" {
  role       = aws_iam_role.bucket_cleaner_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess"
}
resource "aws_iam_role_policy_attachment" "bucket_cleaner_right_basic" {
  role       = aws_iam_role.bucket_cleaner_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

// Eventbridge Rule for triggering bucket cleaner lambda
resource "aws_cloudwatch_event_rule" "invoke_cleaner" {
  name = cloud_trigger_bucket_cleaner_2
  schedule_expression = "cron(5 0 * * ? *)"
  
}

// add bucket cleaner lambda as target for Eventbridge rule
resource "aws_cloudwatch_event_target" "invoke_cleaner" {
  rule = aws_cloudwatch_event_rule.invoke_cleaner_2.id
  arn = aws_lambda_function.bucket_cleaner.arn
}

// give eventbridge rule the correct access rights
resource "aws_lambda_permission" "allow_invoke_cleaner" {
  action = "lambda::InvokeFunction"
  function_name = aws_lambda_function.bucket_cleaner
  principal = events.amazonaws.com
  source_arn = aws_cloudwatch_event_rule.invoke_cleaner_2.arn
}

