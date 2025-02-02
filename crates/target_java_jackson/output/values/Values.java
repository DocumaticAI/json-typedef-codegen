// Code generated by jetted for Java + Jackson v0.1.0

package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Map;

public class Values {
    @JsonValue
    private Map<String, String> value;

    public Values() {
    }

    @JsonCreator
    public Values(Map<String, String> value) {
        this.value = value;
    }

    public Map<String, String> getValue() {
        return value;
    }

    public void setValue(Map<String, String> value) {
        this.value = value;
    }
}
