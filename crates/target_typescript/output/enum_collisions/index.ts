/**

 */

export enum RootFooBar {

    /**

     */

	X = "x",

    /**

     */

	Y = "y",

}
/**

 */

export interface RootFoo {

    /**

     */

    bar: RootFooBar;

}
/**

 */

export enum RootFooBar0 {

    /**

     */

	X = "x",

    /**

     */

	Y = "y",

}
/**

 */

export interface Root {

    /**

     */

    foo: RootFoo;

    /**

     */

    foo_bar: RootFooBar0;

}