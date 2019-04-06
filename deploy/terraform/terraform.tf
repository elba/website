provider "aws" {
  access_key = "${var.access_key}"
  secret_key = "${var.secret_key}"
  region     = "${var.region}"
}

resource "aws_instance" "elba-registry" {
  ami                    = "ami-06c43a7df16e8213c"
  instance_type          = "t2.micro"
  key_name               = "${aws_key_pair.key-pair.key_name}"
  vpc_security_group_ids = ["${aws_security_group.web-server.id}"]
}

resource "aws_default_vpc" "default" {}

resource "aws_security_group" "web-server" {
  name   = "web-server"
  vpc_id = "${aws_default_vpc.default.id}"

  ingress {
    protocol    = "tcp"
    from_port   = 22
    to_port     = 22
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    protocol    = "tcp"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    protocol    = "tcp"
    from_port   = 443
    to_port     = 443
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_key_pair" "key-pair" {
  key_name   = "${var.key_pair_name}"
  public_key = "${var.key_pair_public_key}"
}

resource "aws_s3_bucket" "elba-registry" {
  bucket = "${var.s3_bucket_name}"
  region = "${var.region}"
  acl    = "public-read"
  policy = "${data.aws_iam_policy_document.s3-elba-registry-policy.json}"
}

data "aws_iam_policy_document" "s3-elba-registry-policy" {
  statement {
    actions = [
      "s3:DeleteObject",
      "s3:PutObject",
      "s3:PutObjectAcl",
    ]

    resources = [
      "arn:aws:s3:::${var.s3_bucket_name}/tarballs/*",
      "arn:aws:s3:::${var.s3_bucket_name}/readme/*",
    ]

    principals {
      type        = "AWS"
      identifiers = ["${var.arn_user_registry}"]
    }
  }

  statement {
    actions = [
      "s3:DeleteObject",
      "s3:PutObject",
      "s3:PutObjectAcl",
    ]

    resources = [
      "arn:aws:s3:::${var.s3_bucket_name}/public/*",
    ]

    principals {
      type        = "AWS"
      identifiers = ["${var.arn_user_registry_deployer}"]
    }
  }
}

resource "aws_ecr_repository" "elba-registry" {
  name = "elba/registry"
}
