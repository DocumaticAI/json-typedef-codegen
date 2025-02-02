// Code generated by jetted for C# + System.Text.Json v0.1.0

using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JettedE2E
{
    [JsonConverter(typeof(RootUint8JsonConverter))]
    public class RootUint8
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public byte Value { get; set; }
    }

    public class RootUint8JsonConverter : JsonConverter<RootUint8>
    {
        public override RootUint8 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new RootUint8 { Value = JsonSerializer.Deserialize<byte>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, RootUint8 value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<byte>(writer, value.Value, options);
        }
    }
}
