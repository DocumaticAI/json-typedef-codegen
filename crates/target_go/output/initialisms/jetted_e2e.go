// Code generated by jetted for Go v0.1.0. DO NOT EDIT.

package jetted_e2e

type InitialismsNestedIDInitialism struct {
	JSON string `json:"json"`

	Normalword string `json:"normalword"`
}

type Initialisms struct {
	HTTP string `json:"http"`

	ID string `json:"id"`

	NestedIDInitialism InitialismsNestedIDInitialism `json:"nested_id_initialism"`

	UTF8 string `json:"utf8"`

	WordWithEmbeddedIDInitialism string `json:"word_with_embedded_id_initialism"`

	WordWithTrailingInitialismID string `json:"word_with_trailing_initialism_id"`
}