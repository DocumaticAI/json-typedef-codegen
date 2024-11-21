// Code generated by jetted for Java + Jackson v0.1.0

package com.example;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class Root {
    @JsonProperty("override_elements_container")
    private java.util.ArrayList<String> overrideElementsContainer;

    @JsonProperty("override_type_discriminator")
    private Object overrideTypeDiscriminator;

    @JsonProperty("override_type_enum")
    private RootOverrideTypeEnum overrideTypeEnum;

    @JsonProperty("override_type_expr")
    private Object overrideTypeExpr;

    @JsonProperty("override_type_properties")
    private Object overrideTypeProperties;

    @JsonProperty("override_values_container")
    private java.util.HashMap<String, String> overrideValuesContainer;

    public Root() {
    }

    /**
     * Getter for overrideElementsContainer.<p>
     */
    public java.util.ArrayList<String> getOverrideElementsContainer() {
        return overrideElementsContainer;
    }

    /**
     * Setter for overrideElementsContainer.<p>
     */
    public void setOverrideElementsContainer(java.util.ArrayList<String> overrideElementsContainer) {
        this.overrideElementsContainer = overrideElementsContainer;
    }

    /**
     * Getter for overrideTypeDiscriminator.<p>
     */
    public Object getOverrideTypeDiscriminator() {
        return overrideTypeDiscriminator;
    }

    /**
     * Setter for overrideTypeDiscriminator.<p>
     */
    public void setOverrideTypeDiscriminator(Object overrideTypeDiscriminator) {
        this.overrideTypeDiscriminator = overrideTypeDiscriminator;
    }

    /**
     * Getter for overrideTypeEnum.<p>
     */
    public RootOverrideTypeEnum getOverrideTypeEnum() {
        return overrideTypeEnum;
    }

    /**
     * Setter for overrideTypeEnum.<p>
     */
    public void setOverrideTypeEnum(RootOverrideTypeEnum overrideTypeEnum) {
        this.overrideTypeEnum = overrideTypeEnum;
    }

    /**
     * Getter for overrideTypeExpr.<p>
     */
    public Object getOverrideTypeExpr() {
        return overrideTypeExpr;
    }

    /**
     * Setter for overrideTypeExpr.<p>
     */
    public void setOverrideTypeExpr(Object overrideTypeExpr) {
        this.overrideTypeExpr = overrideTypeExpr;
    }

    /**
     * Getter for overrideTypeProperties.<p>
     */
    public Object getOverrideTypeProperties() {
        return overrideTypeProperties;
    }

    /**
     * Setter for overrideTypeProperties.<p>
     */
    public void setOverrideTypeProperties(Object overrideTypeProperties) {
        this.overrideTypeProperties = overrideTypeProperties;
    }

    /**
     * Getter for overrideValuesContainer.<p>
     */
    public java.util.HashMap<String, String> getOverrideValuesContainer() {
        return overrideValuesContainer;
    }

    /**
     * Setter for overrideValuesContainer.<p>
     */
    public void setOverrideValuesContainer(java.util.HashMap<String, String> overrideValuesContainer) {
        this.overrideValuesContainer = overrideValuesContainer;
    }
}
