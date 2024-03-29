using MessagePack;

using Qynit.PulseGen.Schedules;

namespace Qynit.PulseGen.Aot.Models;

[MessagePackObject]
public sealed class SetFrequencyElementDto : ScheduleElementDto
{
    [Key(6)]
    public int ChannelId { get; set; }
    [Key(7)]
    public double Frequency { get; set; }

    public override ScheduleElement GetScheduleElement(ScheduleRequest request)
    {
        return new SetFrequencyElement(ChannelId, Frequency)
        {
            Margin = Margin,
            Alignment = Alignment,
            IsVisible = IsVisible,
            Duration = Duration,
            MaxDuration = MaxDuration,
            MinDuration = MinDuration,
            PulseGenOptions = request.Options?.GetOptions(),
        };
    }
}
