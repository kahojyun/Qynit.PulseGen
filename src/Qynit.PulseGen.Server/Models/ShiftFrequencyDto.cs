﻿using MessagePack;

namespace Qynit.PulseGen.Server.Models;

[MessagePackObject]
public sealed record ShiftFrequencyDto(
    [property: Key(0)] double Time,
    [property: Key(1)] int ChannelId,
    [property: Key(2)] double Frequency) : InstructionDto;
