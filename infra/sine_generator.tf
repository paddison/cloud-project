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
  filename      = "../builds/cloud_sine_generator/bootstrap.zip" // replace with path of artifact
  function_name = var.GENERATOR_LAMBDA
  role          = aws_iam_role.sine_generator_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2" // Amazon Linux 2?

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
  filename      = "../builds/cloud_bucket_cleaner/bootstrap.zip" // replace with path of artifact
  function_name = var.CLEANER_LAMBDA
  role          = aws_iam_role.bucket_cleaner_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2" // Amazon Linux 2?

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


