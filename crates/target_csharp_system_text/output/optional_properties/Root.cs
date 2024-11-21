// Code generated by jetted for C# + System.Text.Json v0.1.0

using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace JettedE2E
{
    public class Root
    {
        [JsonPropertyName("bar")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public IList<string> Bar { get; set; }

        [JsonPropertyName("baz")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public bool? Baz { get; set; }

        [JsonPropertyName("foo")]
        [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)]
        public string Foo { get; set; }
    }
}
