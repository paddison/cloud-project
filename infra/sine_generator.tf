// Lambdas

# # # # # 
# sine generator
# # # # #
//the lambda service role
resource "aws_iam_role" "sine_generator_role" {
  name = "sine_generator_role" // replace with variable?
}
//the function and its config
resource "aws_lambda_function" "sine_generator" {
  filename      = "" // replace with path of artifact
  function_name = "sine_generator" // replace with variable
  role          = aws_iam_role.sine_generator_role.arn
  handler       = "bootstrap"
  runtime       = "provided" // Amazon Linux 2?

  ephemeral_storage {
    size = 512 # Min 512 MB and the Max 10240 MB
  }
}
//assign rights to lambda
resource "aws_iam_role_policy_attachment" "sine_generator_right" {
  role       = aws_iam_role.sine_generator_role.name
  policy_arn = aws_iam_policy.dynamodb_write_wave_table_policy.arn
}
resource "aws_iam_role_policy_attachment" "sine_generator_right" {
  role       = aws_iam_role.sine_generator_role.name
  policy_arn = "arn:aws:iam::aws:policy/AWSLambdaExecute"
}

# # # # # 
# bucket cleaner
# # # # #
//the lambda service role
resource "aws_iam_role" "bucket_cleaner_role" {
  name = "bucket_cleaner_role" // replace with variable?
}
//the function and its config
resource "aws_lambda_function" "bucket_cleaner" {
  filename      = "" // replace with path of artifact
  function_name = "bucket_cleaner" // replace with variable
  role          = aws_iam_role.bucket_cleaner_role.arn
  handler       = "bootstrap"
  runtime       = "provided" // Amazon Linux 2?

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


