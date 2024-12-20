#---
# Provider Configuration
#---

provider "aws" {
  region = "us-west-2"
}

terraform {
  backend "s3" {
    bucket = "eks-terraform-shared-state"
    key    = "prod/us-west-2/apps/dino-park-packs-prod/terraform.tfstate"
    region = "us-west-2"
  }
}

