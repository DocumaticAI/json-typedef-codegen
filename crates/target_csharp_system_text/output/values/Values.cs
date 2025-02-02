// Code generated by jetted for C# + System.Text.Json v0.1.0

using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JettedE2E
{
    [JsonConverter(typeof(ValuesJsonConverter))]
    public class Values
    {
        /// <summary>
        /// The underlying data being wrapped.
        /// </summary>
        public IDictionary<string, string> Value { get; set; }
    }

    public class ValuesJsonConverter : JsonConverter<Values>
    {
        public override Values Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            return new Values { Value = JsonSerializer.Deserialize<IDictionary<string, string>>(ref reader, options) };
        }

        public override void Write(Utf8JsonWriter writer, Values value, JsonSerializerOptions options)
        {
            JsonSerializer.Serialize<IDictionary<string, string>>(writer, value.Value, options);
        }
    }
}
