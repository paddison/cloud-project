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
  name           = "wave_file"
  billing_mode   = "PROVISIONED"
  read_capacity  = 20
  write_capacity = 20
  hash_key       = "id"
  range_key      = "GameTitle"

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "request_id"
    type = "S"
  }

  attribute {
    name = "date"
    type = "S"
  }

  attribute {
    name = "time"
    type = "S"
  }

  attribute {
    name = "is_downloaded"
    type = "BOOL"
  }

  attribute {
    name = "specs"
    type = "M"
  }

  global_secondary_index {
    name               = "date-time-index"
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
  bucket = "cloud-wav-file-bucket"

}

# todo: create image from current ec2 instance
// EC2 VM 
data "aws_ami" "amazon_linux_ami" {
  most_recent = true

  filter {
    name   = "name"
    values = ["amzn2-ami-kernel-5.10-hvm-2.0.20220606.1-x86_64-gp2"] #Amazon Linux
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }
}

resource "aws_instance" "waveBuilder_webserver" {
  ami           = data.aws_ami.amazon_linux_ami.id
  instance_type = "t2.micro"

  tags = {
    Name = "HelloWorld"
  }
}
