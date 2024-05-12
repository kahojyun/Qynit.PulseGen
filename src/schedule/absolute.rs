use std::sync::OnceLock;

use anyhow::{bail, Result};

use crate::{
    quant::{ChannelId, Time},
    schedule::{merge_channel_ids, ElementRef, Measure, Visit, Visitor},
};

use super::{Arrange, Arranged};

#[derive(Debug, Clone)]
pub(crate) struct AbsoluteEntry {
    time: Time,
    element: ElementRef,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Absolute {
    children: Vec<AbsoluteEntry>,
    channel_ids: Vec<ChannelId>,
    measure_result: OnceLock<Time>,
}

impl AbsoluteEntry {
    pub(crate) fn new(element: ElementRef) -> Self {
        Self {
            time: Time::ZERO,
            element,
        }
    }

    pub(crate) fn with_time(mut self, time: Time) -> Result<Self> {
        if !time.value().is_finite() {
            bail!("Invalid time {:?}", time);
        }
        self.time = time;
        Ok(self)
    }
}

impl Absolute {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with_children(mut self, children: Vec<AbsoluteEntry>) -> Self {
        let channel_ids = merge_channel_ids(children.iter().map(|e| e.element.variant.channels()));
        self.children = children;
        self.channel_ids = channel_ids;
        self
    }

    fn measure_result(&self) -> &Time {
        self.measure_result
            .get_or_init(|| measure_absolute(self.children.iter().map(|e| (&e.element, e.time))))
    }
}

impl Measure for Absolute {
    fn measure(&self) -> Time {
        *self.measure_result()
    }

    fn channels(&self) -> &[ChannelId] {
        &self.channel_ids
    }
}

impl Visit for Absolute {
    fn visit<V>(&self, visitor: &mut V, time: Time, duration: Time) -> Result<()>
    where
        V: Visitor,
    {
        visitor.visit_absolute(self, time, duration)?;
        for AbsoluteEntry {
            time: offset,
            element,
        } in &self.children
        {
            element.visit(visitor, offset + time, element.measure())?;
        }
        Ok(())
    }
}

impl<'a> Arrange<'a> for Absolute {
    fn arrange(
        &'a self,
        time: Time,
        _duration: Time,
    ) -> impl Iterator<Item = Arranged<&'a ElementRef>> {
        self.children.iter().map(
            move |AbsoluteEntry {
                      time: offset,
                      element,
                  }| {
                Arranged {
                    duration: element.measure(),
                    item: element,
                    offset: time + offset,
                }
            },
        )
    }
}

fn measure_absolute<I, M>(children: I) -> Time
where
    I: IntoIterator<Item = (M, Time)>,
    M: Measure,
{
    children
        .into_iter()
        .map(|(child, offset)| offset + child.measure())
        .max()
        .unwrap_or(Time::ZERO)
}
