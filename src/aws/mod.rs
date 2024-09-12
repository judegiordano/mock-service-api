use aws_config::{BehaviorVersion, SdkConfig};

pub async fn config() -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest()).load().await
}
