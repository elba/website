provider "aws" {
  access_key = "${var.access_key}"
  secret_key = "${var.secret_key}"
  region     = "${var.region}"
}

provider "aws" {
  access_key = "${var.access_key}"
  secret_key = "${var.secret_key}"
  region     = "us-east-1"
  alias      = "use1"
}

resource "aws_key_pair" "key-pair" {
  key_name   = "${var.key_pair_name}"
  public_key = "${var.key_pair_public_key}"
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

  ingress {
    protocol    = "tcp"
    from_port   = 8080
    to_port     = 8080
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    protocol    = "icmp"
    from_port   = -1
    to_port     = -1
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    protocol    = "-1"
    from_port   = 0
    to_port     = 0
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_s3_bucket" "elba-registry" {
  bucket = "${var.s3_bucket_registry_name}"
  region = "${var.region}"
  acl    = "public-read"
  policy = "${data.aws_iam_policy_document.s3-elba-registry-policy.json}"

  cors_rule {
    allowed_origins = ["*"]
    allowed_methods = ["HEAD", "GET"]
    max_age_seconds = 3000
  }
}

resource "aws_s3_bucket" "elba-website-root" {
  bucket = "${var.s3_bucket_website_root_name}"
  region = "${var.region}"
  acl    = "public-read"
  policy = "${data.aws_iam_policy_document.s3-elba-website-policy.json}"

  website {
    index_document = "index.html"
    error_document = "index.html"
  }
}

resource "aws_s3_bucket" "elba-website-www" {
  bucket = "${var.s3_bucket_website_www_name}"
  region = "${var.region}"
  acl    = "public-read"

  website {
    redirect_all_requests_to = "${var.domain_website_root}"
  }
}

data "aws_iam_policy_document" "s3-elba-registry-policy" {
  statement {
    actions = [
      "s3:DeleteObject",
      "s3:PutObject",
      "s3:PutObjectAcl",
    ]

    resources = [
      "arn:aws:s3:::${var.s3_bucket_registry_name}/tarballs/*",
      "arn:aws:s3:::${var.s3_bucket_registry_name}/readmes/*",
    ]

    principals {
      type        = "AWS"
      identifiers = ["${var.arn_user_registry}"]
    }
  }

  statement {
    actions = [
      "s3:GetObject",
    ]

    resources = [
      "arn:aws:s3:::${var.s3_bucket_registry_name}/tarballs/*",
      "arn:aws:s3:::${var.s3_bucket_registry_name}/readmes/*",
    ]

    principals {
      type        = "*"
      identifiers = ["*"]
    }
  }
}

data "aws_iam_policy_document" "s3-elba-website-policy" {
  statement {
    actions = [
      "s3:DeleteObject",
      "s3:PutObject",
      "s3:PutObjectAcl",
    ]

    resources = [
      "arn:aws:s3:::${var.s3_bucket_website_root_name}/*",
    ]

    principals {
      type        = "AWS"
      identifiers = ["${var.arn_user_deployer}"]
    }
  }

  statement {
    actions = [
      "s3:GetObject",
    ]

    resources = [
      "arn:aws:s3:::${var.s3_bucket_website_root_name}/*",
    ]

    principals {
      type        = "*"
      identifiers = ["*"]
    }
  }
}

resource "aws_ecr_repository" "elba-registry" {
  name = "elba/registry"
}

locals {
  cdn-website-root-origin-id = "s3-elba-website-root"
  cdn-website-www-origin-id  = "s3-elba-website-www"
  cdn-registry-origin-id     = "s3-elba-registry"
}

resource "aws_cloudfront_distribution" "cdn-registry" {
  origin {
    domain_name = "${aws_s3_bucket.elba-registry.bucket_regional_domain_name}"
    origin_id   = "${local.cdn-registry-origin-id}"
  }

  enabled         = true
  is_ipv6_enabled = true

  default_cache_behavior {
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "${local.cdn-registry-origin-id}"

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }
    }

    min_ttl                = 0
    default_ttl            = 300
    max_ttl                = 3600
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  price_class = "PriceClass_All"

  viewer_certificate {
    cloudfront_default_certificate = true
  }
}

resource "aws_cloudfront_distribution" "cdn-website-root" {
  origin {
    domain_name = "${aws_s3_bucket.elba-website-root.website_endpoint}"
    origin_id   = "${local.cdn-website-root-origin-id}"

    custom_origin_config {
      http_port              = "80"
      https_port             = "443"
      origin_protocol_policy = "http-only"
      origin_ssl_protocols   = ["TLSv1", "TLSv1.1", "TLSv1.2"]
    }
  }

  enabled         = true
  is_ipv6_enabled = true

  aliases = ["${var.domain_website_root}"]

  default_cache_behavior {
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "${local.cdn-website-root-origin-id}"

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }
    }

    min_ttl                = 0
    default_ttl            = 300
    max_ttl                = 3600
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  price_class = "PriceClass_All"

  viewer_certificate {
    ssl_support_method  = "sni-only"
    acm_certificate_arn = "${aws_acm_certificate_validation.cert-website.certificate_arn}"
  }
}

resource "aws_cloudfront_distribution" "cdn-website-www" {
  origin {
    domain_name = "${aws_s3_bucket.elba-website-www.website_endpoint}"
    origin_id   = "${local.cdn-website-www-origin-id}"

    custom_origin_config {
      http_port              = "80"
      https_port             = "443"
      origin_protocol_policy = "http-only"
      origin_ssl_protocols   = ["TLSv1", "TLSv1.1", "TLSv1.2"]
    }
  }

  enabled         = true
  is_ipv6_enabled = true

  aliases = ["${var.domain_website_www}"]

  default_cache_behavior {
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "${local.cdn-website-www-origin-id}"

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }
    }

    min_ttl                = 0
    default_ttl            = 300
    max_ttl                = 3600
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  price_class = "PriceClass_All"

  viewer_certificate {
    ssl_support_method  = "sni-only"
    acm_certificate_arn = "${aws_acm_certificate_validation.cert-website.certificate_arn}"
  }
}

data "aws_route53_zone" "zone" {
  name         = "${var.domain_zone}"
  private_zone = false
}

resource "aws_acm_certificate" "cert-website" {
  domain_name               = "${var.domain_website_root}"
  subject_alternative_names = ["${var.domain_website_www}"]
  validation_method         = "DNS"
  provider                  = "aws.use1"
}

resource "aws_acm_certificate_validation" "cert-website" {
  certificate_arn         = "${aws_acm_certificate.cert-website.arn}"
  validation_record_fqdns = ["${aws_route53_record.cert-validation-website.fqdn}", "${aws_route53_record.cert-validation-website-alt1.fqdn}"]
  provider                = "aws.use1"
}

resource "aws_route53_record" "cert-validation-website" {
  name    = "${aws_acm_certificate.cert-website.domain_validation_options.0.resource_record_name}"
  type    = "${aws_acm_certificate.cert-website.domain_validation_options.0.resource_record_type}"
  zone_id = "${data.aws_route53_zone.zone.id}"
  records = ["${aws_acm_certificate.cert-website.domain_validation_options.0.resource_record_value}"]
  ttl     = 60
}

resource "aws_route53_record" "cert-validation-website-alt1" {
  name    = "${aws_acm_certificate.cert-website.domain_validation_options.1.resource_record_name}"
  type    = "${aws_acm_certificate.cert-website.domain_validation_options.1.resource_record_type}"
  zone_id = "${data.aws_route53_zone.zone.id}"
  records = ["${aws_acm_certificate.cert-website.domain_validation_options.1.resource_record_value}"]
  ttl     = 60
}

resource "aws_route53_record" "website-www" {
  zone_id = "${data.aws_route53_zone.zone.zone_id}"
  name    = "${var.domain_website_www}"
  type    = "CNAME"
  ttl     = "300"
  records = ["${aws_cloudfront_distribution.cdn-website-www.domain_name}"]
}

resource "aws_route53_record" "website-root" {
  zone_id = "${data.aws_route53_zone.zone.zone_id}"
  name    = "${var.domain_website_root}"
  type    = "A"

  alias {
    name                   = "${aws_cloudfront_distribution.cdn-website-root.domain_name}"
    zone_id                = "${aws_cloudfront_distribution.cdn-website-root.hosted_zone_id}"
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "registry" {
  zone_id = "${data.aws_route53_zone.zone.zone_id}"
  name    = "${var.domain_registry}"
  type    = "A"
  ttl     = "300"
  records = ["${aws_instance.elba-registry.public_ip}"]
}
