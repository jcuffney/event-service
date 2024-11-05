# Infra

This project is setup to run on AWS Lambda.

## Steps

1. Create a S3 bucket
2. Upload zipfile so it exists when the lambda is created
3. Deploy remaining infra

```
PROJECT=cuffney
BRANCH=main
SERVICE=event-service
AWS_REGION=us-east-1
aws s3api create-bucket --bucket "$PROJECT-$BRANCH-$SERVICE-source" --region $AWS_REGION
```

```
TARGET=x86_64-unknown-linux-gnu
cargo build --release --target $TARGET

cd target/$TARGET/release
mv your-binary-name bootstrap
zip lambda.zip bootstrap
aws s3 cp lambda.zip s3://<your-s3-bucket-name>/event-service-onEvent.zip
```

```
PROJECT=cuffney
BRANCH=main
SERVICE=event-service
SOURCE_BUCKET_ARN="arn:aws:s3:::$PROJECT-$BRANCH-$SERVICE-source"
aws cloudformation deploy \
    --template-file ./template.yml \
    --stack-name "$PROJECT-$BRANCH-$SERVICE" \
    --capabilities CAPABILITY_NAMED_IAM \
    --parameter-overrides Project=$PROJECT Branch=$BRANCH SERVICE=$SERVICE SourceBucketArn=$SOURCE_BUCKET_ARN 
```

## Cleanup

```
aws cloudformation delete

aws s3 
```