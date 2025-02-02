// Code generated by jetted for Java + Jackson v0.1.0

package com.example;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public class Geojson {
    @JsonValue
    private GeojsonObject value;

    public Geojson() {
    }

    @JsonCreator
    public Geojson(GeojsonObject value) {
        this.value = value;
    }

    public GeojsonObject getValue() {
        return value;
    }

    public void setValue(GeojsonObject value) {
        this.value = value;
    }
}
