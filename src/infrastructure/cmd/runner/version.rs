use async_trait::async_trait;

use crate::{
    infrastructure::{
        cmd::{parser::Parser, runner::Executor},
        config::env::OpCommand,
    },
    service::{version::VersionResult, Runner},
};

#[derive(Clone)]
pub struct VersionRunner<E, P>
where
    E: Executor + Send + Sync,
    P: Parser<Item = VersionResult> + Send + Sync,
{
    command: OpCommand,
    executor: E,
    parser: P,
}

impl<E, P> VersionRunner<E, P>
where
    E: Executor + Send + Sync,
    P: Parser<Item = VersionResult> + Send + Sync,
{
    pub fn new(command: &OpCommand, executor: E, parser: P) -> Self {
        let command = command.to_owned();
        Self {
            command,
            executor,
            parser,
        }
    }

    async fn version(&self) -> anyhow::Result<VersionResult> {
        let output = self.executor.output(&self.command, &["show", "version"]).await?;
        let result = self.parser.parse(&output)?;
        Ok(result)
    }
}

#[async_trait]
impl<E, P> Runner for VersionRunner<E, P>
where
    E: Executor + Send + Sync,
    P: Parser<Item = VersionResult> + Send + Sync,
{
    type Item = VersionResult;

    async fn run(&self) -> anyhow::Result<Self::Item> {
        self.version().await
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use cool_asserts::assert_matches;
    use indoc::indoc;
    use mockall::{mock, predicate::eq};

    use crate::{domain::version::Version, infrastructure::cmd::runner::MockExecutor};

    use super::*;

    mock! {
        VersionParser {}

        impl Parser for VersionParser {
            type Item = VersionResult;

            fn parse(&self, input: &str) -> anyhow::Result<<Self as Parser>::Item>;
        }
    }

    #[tokio::test]
    async fn version() {
        let command = OpCommand::from("/opt/vyatta/bin/vyatta-op-cmd-wrapper".to_string());
        let output = indoc! {"
            Version:      v2.0.6
            Build ID:     5208541
            Build on:     01/02/06 15:04
            Copyright:    2012-2018 Ubiquiti Networks, Inc.
            HW model:     EdgeRouter X 5-Port
            HW S/N:       000000000000
            Uptime:       01:00:00 up  1:00,  1 user,  load average: 1.00, 1.00, 1.00
        "};

        let mut mock_executor = MockExecutor::new();
        mock_executor
            .expect_output()
            .times(1)
            .returning(|command, args| {
                match (command, args) {
                    ("/opt/vyatta/bin/vyatta-op-cmd-wrapper", &["show", "version"]) => Ok(output.to_string()),
                    _ => panic!("unexpected args"),
                }
            });

        let mut mock_parser = MockVersionParser::new();
        mock_parser
            .expect_parse()
            .times(1)
            .with(eq(output))
            .returning(|_| Ok(Version {
                version: "v2.0.6".to_string(),
                build_id: "5208541".to_string(),
                build_on: NaiveDate::from_ymd(2006, 1, 2).and_hms(15, 4, 0),
                copyright: "2012-2018 Ubiquiti Networks, Inc.".to_string(),
                hw_model: "EdgeRouter X 5-Port".to_string(),
                hw_serial_number: "000000000000".to_string(),
                uptime: "01:00:00 up  1:00,  1 user,  load average: 1.00, 1.00, 1.00".to_string(),
            }));

        let runner = VersionRunner::new(&command, mock_executor, mock_parser);
        assert_matches!(
            runner.run().await,
            Ok(version) if version == Version {
                version: "v2.0.6".to_string(),
                build_id: "5208541".to_string(),
                build_on: NaiveDate::from_ymd(2006, 1, 2).and_hms(15, 4, 0),
                copyright: "2012-2018 Ubiquiti Networks, Inc.".to_string(),
                hw_model: "EdgeRouter X 5-Port".to_string(),
                hw_serial_number: "000000000000".to_string(),
                uptime: "01:00:00 up  1:00,  1 user,  load average: 1.00, 1.00, 1.00".to_string(),
            },
        );
    }
}
