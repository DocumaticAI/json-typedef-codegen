using System;
using System.Text.Json;
using System.Text.Json.Serialization;
namespace JtdCodegenE2E
{
    [JsonConverter(typeof(Root0JsonConverter))]
    public enum Root0
    {
        Bar,
        Baz,
        Foo,
    }
    public class Root0JsonConverter : JsonConverter<Root0>
    {
        public override Root0 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            string value = JsonSerializer.Deserialize<string>(ref reader, options);
            switch (value)
            {
                case "Bar":
                    return Root0.Bar;
                case "Baz":
                    return Root0.Baz;
                case "Foo":
                    return Root0.Foo;
                default:
                    throw new ArgumentException(String.Format("Bad Root0 value: {0}", value));
            }
        }
        public override void Write(Utf8JsonWriter writer, Root0 value, JsonSerializerOptions options)
        {
            switch (value)
            {
                case Root0.Bar:
                    JsonSerializer.Serialize<string>(writer, "Bar", options);
                    return;
                case Root0.Baz:
                    JsonSerializer.Serialize<string>(writer, "Baz", options);
                    return;
                case Root0.Foo:
                    JsonSerializer.Serialize<string>(writer, "Foo", options);
                    return;
            }
        }
    }
}
