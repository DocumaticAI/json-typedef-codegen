// Code generated by jetted for C# + System.Text.Json v0.1.0

using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JettedE2E
{
    [JsonConverter(typeof(RootNullableTimestampJsonConverter))]
    public class RootNullableTimestamp
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public DateTimeOffset? Value { get; set; }
    }

    public class RootNullableTimestampJsonConverter : JsonConverter<RootNullableTimestamp>
    {
        public override RootNullableTimestamp Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new RootNullableTimestamp { Value = JsonSerializer.Deserialize<DateTimeOffset?>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, RootNullableTimestamp value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<DateTimeOffset?>(writer, value.Value, options);
        }
    }
}