
# Table of Contents

1.  [tera](#org55bae48)
    1.  [general](#orgaa23aef)
        1.  [downloads:](#org6ba8ce0)
    2.  [pros](#org9fc1ff3)
    3.  [cons](#org4dcc781)
2.  [gtmpl](#org2e000c9)
    1.  [general](#orgbae2e61)
        1.  [downloads](#orgff4617f)
    2.  [pros](#org51c449b)
    3.  [cons](#orged2d9de)
3.  [askama](#org204926b)
    1.  [general](#orgab98417)
        1.  [downloads](#org9ec326a)
    2.  [pros](#org5bbc3d6)
    3.  [cons](#orgb86fef5)
4.  [final notes:](#org81af531)


<a id="org55bae48"></a>

# tera


<a id="orgaa23aef"></a>

## general

-   jinja2 temlating engine
-   MIT license
-   2.7k github stars
-   Updated: about 2 months ago


<a id="org6ba8ce0"></a>

### downloads:

-   All-Time: 5,664,947
-   Recent: 773,985


<a id="org9fc1ff3"></a>

## pros

-   widely used
-   templating language used alot in python
-   more active community
-   tera-cli
-   simple to use
-   good documentation <https://tera.netlify.app/docs>


<a id="org4dcc781"></a>

## cons

-   some type checking but happens in runtime


<a id="org2e000c9"></a>

# gtmpl


<a id="orgbae2e61"></a>

## general

-   gotemplate engine
-   MIT license
-   66 github stars
-   Updated: almost 2 years ago


<a id="orgff4617f"></a>

### downloads

-   All-Time: 130,836
-   Recent: 16,121


<a id="org51c449b"></a>

## pros

-   go-templates are specified in the doc (may be just a recommendation)
-   better interoperability with go projects
-   used by other project
-   jan gottschick recommended


<a id="orged2d9de"></a>

## cons

-   seems to change api/struct funcs more often -> upgrading may break has broken things
-   still in 0.7 release and last commit 2 years ago (seems dead?)
-   not used alot
-   less documentation
-   some type checking but happens in runtime
-   very little documentation <https://docs.rs/crate/gtmpl/latest>


<a id="org204926b"></a>

# askama


<a id="orgab98417"></a>

## general

-   jinja2
-   Apache2, MIT dual license
-   2.2k github stars
-   Updated: 2 months ago


<a id="org9ec326a"></a>

### downloads

-   All-Time: 3,072,505
-   Recent: 549,705


<a id="org5bbc3d6"></a>

## pros

-   uses macros under the hood, **&ldquo;generated&rdquo; code type safety assured while compiling generator!!!**
-   specifically meant for generating rust code from templates
-   very good documentation <https://djc.github.io/askama/>
-   slightly faster (tho that shouldnt really matter)


<a id="orgb86fef5"></a>

## cons

-   may be harder to write
-   not as matured as tera


<a id="org81af531"></a>

# final notes:

-   all seem to be feature equivalent.
-   they support inheritance, filters, loops &#x2026;
-   all are similar in performance and minimal performance gains shouldn&rsquo;t matter during the generation process

