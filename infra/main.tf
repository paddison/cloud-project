terraform {
  required_providers {
    local = {
      source = "hashicorp/local"
      version = "2.2.3"
    }
  }
}


provider "aws" {
  region     = "eu-central-1"
}

/*
resources:

//Lambdas:

sine_generator
main_lambda
wave_delivery_service
bucket_cleaner

//API Gateways:

MainLambdaOps
wave_delivery_service-API

//EC2 VM

WaveBuilder_Webserver

//S3

cloud-wav-file-bucket
bucket_policy

//DynamoDB

wave_file (table)

//IAM Roles

sine_generator_role
main_lambda_role
wave_delivery_service_role
bucket_cleaner_role

//IAM Policies

read_wave_files_db_policy
get_wave_files_bucket_policy

*/

// DynamoDB Table 'wave_file'
resource "aws_dynamodb_table" "wave_file_table" {
  name           = var.TABLE_NAME
  billing_mode   = "PROVISIONED"
  read_capacity  = 20
  write_capacity = 20
  hash_key       = "id"

  attribute {
    name = "id"
    type = "S"
  }

  # attribute {
  #   name = "request_id"
  #   type = "S"
  # }

  attribute {
    name = "date"
    type = "S"
  }

  attribute {
    name = "time"
    type = "S"
  }

  # attribute {
  #   name = "is_downloaded"
  #   type = "S"
  # }

  # attribute {
  #   name = "specs"
  #   type = "S"
  # }

  global_secondary_index {
    name               = var.GLOBAL_INDEX
    hash_key           = "date"
    range_key          = "time"
    write_capacity     = 10
    read_capacity      = 10
    projection_type    = "INCLUDE"
    non_key_attributes = ["is_downloaded"]
  }
}

// S3 Bucket 'cloud-wav-file-bucket'
resource "aws_s3_bucket" "cloud-wav-file-bucket" {
  bucket = var.BUCKET_NAME

}

// S3 Bucket 'react-website-bucket'
resource "aws_s3_bucket" "react-website-bucket" {
  bucket = "cloud-react-website-bucket"


}

// the website config for the 'cloud-react-website-bucket'
resource "aws_s3_bucket_website_configuration" "react_website" {
  bucket = aws_s3_bucket.react-website-bucket.bucket

  index_document {
    suffix = "index.html"
  }
}

// enables the public access of the 'cloud-react-website-bucket'
resource "aws_s3_bucket_public_access_block" "public_access" {
  bucket = aws_s3_bucket.react-website-bucket.id

  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

// //
// files to upload to the 'cloud-react-website-bucket'

# resource "aws_s3_object" "object" {
#   for_each = fileset("../cloud_s3_frontend/builds/", "*")

#   bucket = aws_s3_bucket.react-website-bucket.name
#   key    = each.value
#   source = "../cloud_s3_frontend/builds/${each.value}"
#   etag   = filemd5("../cloud_s3_frontend/builds/${each.value}")

#   depends_on = [
#     react_build_script
#   ]
# }

# resource "null_resource" "react_build_script" {
#    provisioner "local-exec" {
#         command     = "node --version; npm --version; cd ../cloud_s3_frontend/; npm install; npm build;"
#         # command     = "brew install nodejs; node --version; npm --version; cd ../cloud_s3_frontend/; npm install; npm build;"
#     }
# }

