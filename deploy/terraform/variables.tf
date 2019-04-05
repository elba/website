variable "access_key" {}
variable "secret_key" {}

variable "region" {
  default = "ap-northeast-1"
}

variable "key_pair_name" {}
variable "key_pair_public_key" {}

variable "arn_user_registry" {}
variable "arn_user_registry_deployer" {}

variable "s3_bucket_name" {
  default = "elba-registry"
}
