variable "access_key" {}
variable "secret_key" {}

variable "region" {
  default = "ap-northeast-1"
}

variable "key_pair_name" {}
variable "key_pair_public_key" {}

variable "arn_user_registry" {}
variable "arn_user_deployer" {}

variable "s3_bucket_registry_name" {
  default = "elba-registry"
}

variable "s3_bucket_website_www_name" {
  default = "www.elba.pub"
}

variable "s3_bucket_website_root_name" {
  default = "elba.pub"
}

variable "domain_zone" {
  default = "elba.pub"
}

variable "domain_website_www" {
  default = "www.elba.pub"
}

variable "domain_website_root" {
  default = "elba.pub"
}

variable "domain_registry" {
  default = "api.elba.pub"
}
