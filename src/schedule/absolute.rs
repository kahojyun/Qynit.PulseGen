use anyhow::{bail, Result};
use itertools::Itertools as _;

use super::{
    arrange, measure, ArrangeContext, ArrangeResult, ArrangeResultVariant, ElementRef,
    MeasureContext, MeasureResult, MeasureResultVariant, Schedule,
};

#[derive(Debug, Clone)]
pub struct AbsoluteEntry {
    time: f64,
    element: ElementRef,
}

impl AbsoluteEntry {
    pub fn new(element: ElementRef) -> Self {
        Self { time: 0.0, element }
    }

    pub fn with_time(mut self, time: f64) -> Result<Self> {
        if !time.is_finite() {
            bail!("Invalid time {}", time);
        }
        self.time = time;
        Ok(self)
    }

    pub fn time(&self) -> &f64 {
        &self.time
    }

    pub fn element(&self) -> &ElementRef {
        &self.element
    }
}

#[derive(Debug, Clone)]
pub struct Absolute {
    children: Vec<AbsoluteEntry>,
    channel_ids: Vec<usize>,
}

impl Absolute {
    pub fn new() -> Self {
        Self {
            children: vec![],
            channel_ids: vec![],
        }
    }

    pub fn with_children(mut self, children: Vec<AbsoluteEntry>) -> Self {
        let channel_ids = children
            .iter()
            .flat_map(|e| e.element.variant.channels())
            .copied()
            .unique()
            .collect();
        self.children = children;
        self.channel_ids = channel_ids;
        self
    }

    pub fn children(&self) -> &[AbsoluteEntry] {
        &self.children
    }
}

impl Schedule for Absolute {
    fn measure(&self, context: &MeasureContext) -> MeasureResult {
        let mut max_time: f64 = 0.0;
        let mut measured_children = vec![];
        for e in &self.children {
            let measured_child = measure(e.element.clone(), context.max_duration);
            max_time = max_time.max(e.time + measured_child.duration);
            measured_children.push(measured_child);
        }
        MeasureResult(max_time, MeasureResultVariant::Multiple(measured_children))
    }

    fn arrange(&self, context: &ArrangeContext) -> Result<ArrangeResult> {
        let measured_children = match &context.measured_self.data {
            MeasureResultVariant::Multiple(v) => v,
            _ => bail!("Invalid measure data"),
        };
        let arranged_children = self
            .children
            .iter()
            .map(|e| e.time)
            .zip(measured_children.iter())
            .map(|(t, mc)| arrange(mc, t, mc.duration, context.options))
            .collect::<Result<_>>()?;
        Ok(ArrangeResult(
            context.final_duration,
            ArrangeResultVariant::Multiple(arranged_children),
        ))
    }

    fn channels(&self) -> &[usize] {
        &self.channel_ids
    }
}
