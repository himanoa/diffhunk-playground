use structopt::StructOpt;
use git2::{Repository, Blob, Error, BranchType, Oid, DiffHunk};

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "r", long)]
    repository: Option<String>,

    #[structopt(short = "b", long)]
    branch: String,
}

fn branch_oid<'a>(repo: &'a Repository, branch_name: &String) -> Option<Oid> {
    if let Ok(branch) = repo.find_branch(branch_name, BranchType::Local) {
        return branch.get().target()
    }
    return None
}
fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let path = opt.repository.as_ref().map(|s| &s[..]).unwrap_or(".");
    if let Ok(repo) = Repository::open(path) {
        if let Some(oid) = branch_oid(&repo, &opt.branch) {
            if let Ok(blob) = repo.find_blob(oid) {
                repo.diff_blobs(
                    None,
                    None,
                    Some(&blob),
                    None,
                    None,
                    None,
                    None,
                    Some(&mut |_d, h| print_hunk(h)), 
                    None
                );
            }
        }
    }
}

fn print_hunk(hunk: DiffHunk) -> bool {
    println!("{:#?}", hunk);
    true
}
