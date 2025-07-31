use crate::bfs::test_bfs;
use crate::rcsmart::test_sm;
pub mod bfs;
pub mod rcsmart;



fn main() {

    test_bfs();
    test_sm();
}
