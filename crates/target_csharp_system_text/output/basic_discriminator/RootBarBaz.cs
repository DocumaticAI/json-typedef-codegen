// Code generated by jtd-codegen for C# + System.Text.Json v0.2.0

using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    public class RootBarBaz : Root
    {
        [JsonPropertyName("foo")]
        public string Foo { get => "BAR_BAZ"; }

        [JsonPropertyName("baz")]
        public string Baz { get; set; }
    }
}
