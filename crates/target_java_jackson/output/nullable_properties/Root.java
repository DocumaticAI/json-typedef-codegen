// Code generated by jtd-codegen for Java + Jackson v0.2.0

package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.util.List;

@JsonSerialize
public class Root {
    @JsonProperty("bar")
    private String bar;

    @JsonProperty("baz")
    private List<Boolean> baz;

    @JsonProperty("foo")
    private Boolean foo;

    @JsonProperty("quux")
    private List<Boolean> quux;

    public Root() {
    }

    /**
     * Getter for bar.<p>
     */
    public String getBar() {
        return bar;
    }

    /**
     * Setter for bar.<p>
     */
    public void setBar(String bar) {
        this.bar = bar;
    }

    /**
     * Getter for baz.<p>
     */
    public List<Boolean> getBaz() {
        return baz;
    }

    /**
     * Setter for baz.<p>
     */
    public void setBaz(List<Boolean> baz) {
        this.baz = baz;
    }

    /**
     * Getter for foo.<p>
     */
    public Boolean getFoo() {
        return foo;
    }

    /**
     * Setter for foo.<p>
     */
    public void setFoo(Boolean foo) {
        this.foo = foo;
    }

    /**
     * Getter for quux.<p>
     */
    public List<Boolean> getQuux() {
        return quux;
    }

    /**
     * Setter for quux.<p>
     */
    public void setQuux(List<Boolean> quux) {
        this.quux = quux;
    }
}
