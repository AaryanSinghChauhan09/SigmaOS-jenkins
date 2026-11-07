//! jenkins model — CI/CD automation server.

#[derive(Debug, Clone)]
pub struct Job {
    pub name: String,
    pub status: String,
    pub last_build: Option<u64>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Build {
    pub number: u64,
    pub status: String,
    pub duration: u64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub jobs: Vec<Job>,
    pub builds: Vec<Build>,
}

impl Default for Model {
    fn default() -> Self { Self { jobs: vec![], builds: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_job(&mut self, job: Job) { self.jobs.push(job); }

    pub fn trigger_build(&mut self, job_name: &str) -> Option<u64> {
        let job = self.jobs.iter_mut().find(|j| j.name == job_name)?;
        let num = self.builds.len() as u64 + 1;
        job.last_build = Some(num);
        job.status = "building".into();
        self.builds.push(Build { number: num, status: "running".into(), duration: 0 });
        Some(num)
    }

    pub fn list_jobs(&self) -> &[Job] { &self.jobs }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trigger_build() {
        let mut j = Model::new();
        j.add_job(Job { name: "build".into(), status: "idle".into(), last_build: None, url: "/job/build".into() });
        assert!(j.trigger_build("build").is_some());
        assert_eq!(j.jobs[0].status, "building");
    }
}
