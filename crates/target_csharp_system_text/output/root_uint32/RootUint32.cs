// Code generated by jetted for C# + System.Text.Json v0.1.0

using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JettedE2E
{
    [JsonConverter(typeof(RootUint32JsonConverter))]
    public class RootUint32
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public uint Value { get; set; }
    }

    public class RootUint32JsonConverter : JsonConverter<RootUint32>
    {
        public override RootUint32 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new RootUint32 { Value = JsonSerializer.Deserialize<uint>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, RootUint32 value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<uint>(writer, value.Value, options);
        }
    }
}