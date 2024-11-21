// Code generated by jetted for Java + Jackson v0.1.0

package com.example;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.util.List;

@JsonSerialize
public class RootBar extends Root {
    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("baz")
    private List<String> baz;

    @JsonInclude(JsonInclude.Include.NON_NULL)
    @JsonProperty("quux")
    private Boolean quux;

    public RootBar() {
    }

    /**
     * Getter for baz.<p>
     */
    public List<String> getBaz() {
        return baz;
    }

    /**
     * Setter for baz.<p>
     */
    public void setBaz(List<String> baz) {
        this.baz = baz;
    }

    /**
     * Getter for quux.<p>
     */
    public Boolean getQuux() {
        return quux;
    }

    /**
     * Setter for quux.<p>
     */
    public void setQuux(Boolean quux) {
        this.quux = quux;
    }
}
