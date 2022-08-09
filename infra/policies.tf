# 
resource "aws_iam_policy" "read_and_update_wave_files_db_policy" {
  name = "read_and_update_wave_files_db_policy"

  policy = jsonencode({
    Version = "2022-07-27"
    Statement = [
      {
        Action   = ["dynamodb:GetItem", "dynamodb:Query", "dynamodb:UpdateItem"]
        Effect   = "Allow"
        Resource = aws_dynamodb_table.wave_file_table.arn
      },
    ]
  })

  description = "allows a role to read the data from a table named wave_file"
}

# 
resource "aws_iam_policy" "get_wave_files_bucket_policy" {
  name = "get_wave_files_bucket_policy"

  policy = jsonencode({
    Version = "2022-07-27"
    Statement = [
      {
        Action   = ["s3:ListBucket", "s3:HeadBucket", "s3:GetObject", "s3:HeadObject"]
        Effect   = "Allow"
        Resource = aws_s3_bucket.cloud-wav-file-bucket.arn
      },
    ]
  })

  description = "allows a role to get an object out of a s3 bucket named cloud-wav-file-bucket"
}

# S3 Bucket Policy to allow access 
resource "aws_s3_bucket_policy" "allow_access_from_another_account" {
  bucket = aws_s3_bucket.cloud-wav-file-bucket.id
  policy = data.aws_iam_policy_document.allow_access_to_wave_files.json
}

data "aws_iam_policy_document" "allow_access_to_wave_files" {
  statement {
    principals {
      type        = "AWS"
      identifiers = [] //lambda roles that need access to the bucket: wave_delivery_service, main_lamda, bucket_cleaner
    }

    actions = [
      "s3:GetObject",
      "s3:ListBucket",
      "s3:PutObject",
      "s3:DeleteObject"
    ]

    resources = [
      aws_s3_bucket.cloud-wav-file-bucket.arn,
      "${aws_s3_bucket.cloud-wav-file-bucket.arn}/*",
    ]
  }
}

#
resource "aws_iam_policy" "dynamodb_read_list_everybody_policy" {
  name        = "dynamodb_read_list_everybody_policy"
  description = "My test policy" // replace

  policy = jsonencode({
    Version = "2012-10-17" // replace with current date
    Statement = [
      {
        Action = [
          "dynamodb:GetItem",
        ]
        Effect   = "Allow"
        Resource = "*"
      },
    ]
  })
}

#
resource "aws_iam_policy" "invoke_sine_generator_policy" {
  name        = "invoke_sine_generator_policy"
  description = "My test policy" // replace

  policy = jsonencode({
    Version = "2012-10-17" // replace with current date
    Statement = [
      {
        Action = [
          "lambda:InvokeFunction",
        ]
        Effect   = "Allow"
        Resource = "*" // replace with sine generator lambda arn
      },
    ]
  })
}

#
resource "aws_iam_policy" "dynamodb_write_wave_table_policy" {
  name        = "dynamodb_write_wave_table_policy"
  description = "My test policy" // replace

  policy = jsonencode({
    Version = "2012-10-17" // replace with current date
    Statement = [
      {
        Action = [
          "dynamodb:PutItem",
        ]
        Effect   = "Allow"
        Resource = aws_dynamodb_table.wave_file_table.arn
      },
    ]
  })
}