use crate::ir::common::Description;
use crate::ir::workflow::environment::Env;
use crate::ir::workflow::parameter::Param;
use crate::ir::workflow::pipeline::Pipeline;

/// Top-level workflow definition consumed by the engine control-plane.
///
/// This struct is the canonical IR shape produced from:
/// `workflow "<id>" version="<semver>" { ... }`.
///
/// Control-plane responsibilities:
/// - validate `id`, `version`, and cross references;
/// - expand parameters and strategy declarations into executable attempts;
/// - build a run graph from `pipeline`.
///
/// Data-plane responsibilities:
/// - execute each planned attempt in the selected environment;
/// - stream artifacts referenced by stage outputs.
pub struct Workflow {
    /// Workflow identifier (for example `Somatic-Variant-Calling`).
    pub id: String,
    /// Workflow semantic version.
    pub version: String,
    /// Human-readable workflow purpose.
    pub description: Description,
    /// Global user/config parameters used by templates and stage inputs.
    pub parameters: Vec<Param>,
    /// Environment catalog that stage execution resolves against.
    pub environments: Vec<Env>,
    /// Stage graph definition.
    pub pipeline: Pipeline,
}
