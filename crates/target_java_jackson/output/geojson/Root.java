package com.example;


import com.fasterxml.jackson.annotation.JsonCreator;

import com.fasterxml.jackson.annotation.JsonValue;

/**

 */

public class Root {
    @JsonValue
    private GeojsonObject value;

    public Root() {
    }

    @JsonCreator
    public Root(GeojsonObject value) {
        this.value = value;
    }

    public GeojsonObject getValue() {
        return value;
    }

    public void setValue(GeojsonObject value) {
        this.value = value;
    }
}