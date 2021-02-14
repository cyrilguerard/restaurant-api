use crate::data::entity::MenuItem;
use crate::utils::error::Error;

pub fn find_all<F>(find_items: F) -> Result<Vec<MenuItem>, Error>
where
    F: Fn() -> Result<Vec<MenuItem>, Error>,
{
    find_items()
}
