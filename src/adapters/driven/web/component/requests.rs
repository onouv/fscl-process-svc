use crate::ports::{NewComponentRequest, RequestBuildError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateComponentHttpRequestBody {
    pub project_id: String,
    pub id: String,
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub parent_id: Option<String>,
}

impl CreateComponentHttpRequestBody {
    pub(crate) fn try_into_domain(self) -> Result<NewComponentRequest, RequestBuildError> {
        NewComponentRequest::new(
            self.project_id,
            self.id,
            self.name,
            self.description,
            self.parent_id,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImplementFunctionHttpRequestBody {
    pub function_id: String,
}
