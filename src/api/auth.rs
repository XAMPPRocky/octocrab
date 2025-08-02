use exchange_web_flow_code::ExchangeWebFlowCodeBuilder;

use crate::Octocrab;

pub mod exchange_web_flow_code;

pub struct ExchangeWebFlowCodeHandler<'octo> {
    crab: &'octo Octocrab,
}

impl<'octo> ExchangeWebFlowCodeHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    pub fn exchange_token(&self) -> ExchangeWebFlowCodeBuilder<'_, '_, '_, '_> {
        //TODO: add params
        ExchangeWebFlowCodeBuilder::new(self.crab)
    }
}
