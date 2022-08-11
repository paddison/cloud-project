# 
resource "aws_iam_policy" "read_and_update_wave_files_db_policy" {
  name = "cloud-read-and-update-wave-files-db-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action   = ["dynamodb:GetItem", "dynamodb:Query", "dynamodb:UpdateItem"]
        Effect   = "Allow"
        Resource = "${aws_dynamodb_table.wave_file_table.arn}"
      },
    ]
  })

  description = "allows a role to read the data from a table named wave_files"
}

# 
resource "aws_iam_policy" "get_wave_files_bucket_policy" {
  name = "cloud-get-wave-files-bucket-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action   = ["s3:ListBucket", "s3:HeadBucket", "s3:GetObject", "s3:HeadObject"]
        Effect   = "Allow"
        Resource = "${aws_s3_bucket.cloud-wav-file-bucket.arn}"
      },
    ]
  })

  description = "allows a role to get an object out of a s3 bucket named cloud-wav-file-bucket"
}

# S3 Bucket Policy to allow access to 'cloud-wav-file-bucket'
resource "aws_s3_bucket_policy" "allow_access_from_lambdas" {
  bucket = aws_s3_bucket.cloud-wav-file-bucket.id
  policy = <<EOF
{
    "Version": "2012-10-17",
    "Id": "PolicyAccessFromLambdas",
    "Statement": [
        {
            "Sid": "LambdaAccessAction",
            "Effect": "Allow",
            "Principal": {"AWS": ["${aws_iam_role.wave_delivery_service_role.arn}", "${aws_iam_role.main_lambda_role.arn}", "${aws_iam_role.bucket_cleaner_role.arn}", "${aws_iam_role.sine_generator_role.arn}"]},
            "Action": ["s3:GetObject","s3:PutObject","s3:DeleteObject"],
            "Resource": "${aws_s3_bucket.cloud-wav-file-bucket.arn}/*"
        }
    ]
}
EOF
  #data.aws_iam_policy_document.allow_access_to_wave_files.json
}

data "aws_iam_policy_document" "allow_access_to_wave_files" {
  statement {
    principals {
      type        = "AWS"
      identifiers = [aws_lambda_function.wave_delivery_service.arn, aws_lambda_function.main_lambda.arn, aws_lambda_function.sine_generator.arn, aws_lambda_function.bucket_cleaner.arn] //lambda roles that need access to the bucket: wave_delivery_service, main_lamda, bucket_cleaner
    }

    actions = [
      "s3:GetObject",
      "s3:ListBucket",
      "s3:PutObject",
      "s3:DeleteObject",
      "s3:HeadBucket",
      "s3:HeadObject"
    ]

    resources = [
      aws_s3_bucket.cloud-wav-file-bucket.arn,
      "${aws_s3_bucket.cloud-wav-file-bucket.arn}/*",
    ]
  }
}

# S3 Bucket Policy to allow access to 'cloud-react-website-bucket'
resource "aws_s3_bucket_policy" "allow_access_from_everyone" {
  bucket = aws_s3_bucket.react-website-bucket.id
  policy = <<EOF
{
    "Version": "2012-10-17",
    "Id": "PolicyAccessFromEveryone",
    "Statement": [
        {
            "Sid": "AccessEveryoneActions",
            "Effect": "Allow",
            "Principal": "*",
            "Action": "s3:GetObject",
            "Resource": "${aws_s3_bucket.react-website-bucket.arn}/*"
        }
    ]
}
EOF
}

#
resource "aws_iam_policy" "dynamodb_read_list_everybody_policy" {
  name        = "cloud-dynamodb-read-list-everybody-policy"
  description = "My test policy" // replace

  policy = jsonencode({
    Version = "2012-10-17" 
    Statement = [
      {
        Action = [
          "dynamodb:*",
        ]
        Effect   = "Allow"
        Resource = "*"
      },
    ]
  })
}

#
resource "aws_iam_policy" "invoke_sine_generator_policy" {
  name        = "cloud-invoke-sine-generator-policy"
  description = "My test policy" // replace

  policy = jsonencode({
    Version = "2012-10-17" 
    Statement = [
      {
        Action = [
          "lambda:InvokeFunction",
        ]
        Effect   = "Allow"
        Resource = "${aws_lambda_function.sine_generator.arn}" 
      },
    ]
  })
}

#
resource "aws_iam_policy" "dynamodb_write_wave_table_policy" {
  name        = "cloud-dynamodb-write-wave-table-policy"
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