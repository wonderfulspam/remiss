mod config;
use redmine_api::api::issues::{AssigneeFilter, IssueEssentials, IssuesWrapper, ListIssues};
use redmine_api::api::Redmine;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config()?;
    let redmine = Redmine::new(Url::parse(&config.redmine_url)?, &config.redmine_api_key)?;
    if let Ok(issues) = query_issues(&redmine) {
        println!("{} issues assigned", issues.len());
    } else {
        eprintln!("Could not connect to Redmine");
    }
    Ok(())
}

fn query_issues(redmine: &Redmine) -> Result<Vec<IssueEssentials>, Box<dyn std::error::Error>> {
    let endpoint = ListIssues::builder().assignee(AssigneeFilter::Me).build()?;
    let IssuesWrapper { issues } =
        redmine.json_response_body::<_, IssuesWrapper<IssueEssentials>>(&endpoint)?;
    Ok(issues)
}
