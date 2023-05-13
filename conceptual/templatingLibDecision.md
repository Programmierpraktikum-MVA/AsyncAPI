
# Table of Contents

1.  [tera](#org6eb77df)
    1.  [general](#org5ea2165)
        1.  [downloads:](#org35d61f3)
    2.  [pros](#orgb005db8)
    3.  [cons](#org5c3f77b)
    4.  [Examples:](#org385114d)
2.  [gtmpl](#org4c8367a)
    1.  [general](#org46039ba)
        1.  [downloads](#org80a5b1c)
    2.  [pros](#org1b43a18)
    3.  [cons](#org51a51e4)
    4.  [Examples:](#orgd647256)
3.  [askama](#org0e0dc08)
    1.  [general](#org8936740)
        1.  [downloads](#orga851b07)
    2.  [pros](#orgf11124d)
    3.  [cons](#orgcc0a8c1)
    4.  [Examples](#orga040776)
        1.  [template example](#org9f5563b)
        2.  [code example](#org09c3417)
        3.  [output](#orgbedae95)
4.  [final notes:](#org67e7abe)


<a id="org6eb77df"></a>

# tera


<a id="org5ea2165"></a>

## general

-   jinja2 temlating engine (more common then gotemplate)
-   MIT license
-   2.7k github stars
-   Updated: about 2 months ago


<a id="org35d61f3"></a>

### downloads:

-   All-Time: 5,664,947
-   Recent: 773,985


<a id="orgb005db8"></a>

## pros

-   widely used
-   templating language used alot in python
-   more active community
-   tera-cli
-   simple to use
-   good documentation <https://tera.netlify.app/docs>


<a id="org5c3f77b"></a>

## cons

-   no type checking


<a id="org385114d"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/f6034d5ce5369586e90157f8c3d0e03e5098e8fc/templateTest/src> (doesn&rsquo;t use lazy-statics <https://crates.io/crates/lazy_static> for statics at runtime)


<a id="org4c8367a"></a>

# gtmpl


<a id="org46039ba"></a>

## general

-   gotemplate engine
-   MIT license
-   66 github stars
-   Updated: almost 2 years ago


<a id="org80a5b1c"></a>

### downloads

-   All-Time: 130,836
-   Recent: 16,121


<a id="org1b43a18"></a>

## pros

-   go-templates are specified in the doc (may be just a recommendation)
-   better interoperability with go projects
-   used by other project
-   jan gottschick recommended


<a id="org51a51e4"></a>

## cons

-   seems to change api/struct funcs more often -> upgrading may break has broken things
-   still in 0.7 release and last commit 2 years ago (seems dead?)
-   not used alot
-   no type checking
-   very little documentation <https://docs.rs/crate/gtmpl/latest>
-   documentation doesn&rsquo;t mention filters or inheritance
-   barely used (see github stars)


<a id="orgd647256"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/Niclas>


<a id="org0e0dc08"></a>

# askama


<a id="org8936740"></a>

## general

-   jinja2
-   Apache2, MIT dual license
-   2.2k github stars
-   Updated: 2 months ago


<a id="orga851b07"></a>

### downloads

-   All-Time: 3,072,505
-   Recent: 549,705


<a id="orgf11124d"></a>

## pros

-   uses macros under the hood, checks template type e.g. template file:
    
        {% if test %}
        let var = test;
        {% endif %}
    
    generator file:
    
        #[derive(Template)]
        struct ... {
            test: i32
        }
    
    would cause an error at compile time/ would show up in the ide since the types dont match
    the other ones would just let it slide.
-   generally more compile time(lsp) checks e.g. variable not used in template
-   specifically (but not solely) meant for generating rust code from templates
-   very good documentation <https://djc.github.io/askama/>
-   slightly faster (tho that shouldnt really matter)


<a id="orgcc0a8c1"></a>

## cons

-   not as matured as tera


<a id="orga040776"></a>

## Examples


<a id="org9f5563b"></a>

### template example

templates/pub.rs.jinja: (same as tera)

    use bytes::Bytes;
    use futures::StreamExt;
    
    #[tokio::main]
    async fn main() -> Result<(), async_nats::Error> {
        let client = async_nats::connect("{{ server }}").await?;
    
        let mut subscriber = client.subscribe("{{ subject }}".into()).await?.take(10);
    
        {% if publish  %}
        for _ in 0..10 {
            client.publish("{{ subject }}".into(), "{{ payload }}".into()).await?;
        }
        {% endif %}
        Ok(())
    }


<a id="org09c3417"></a>

### code example

src/main.rs

    use std::{fs::File, io::Write};
    
    use askama::Template;
    
    #[derive(Template)]
    #[template(path = "pub.rs.jinja")]
    struct PublishTemplate<'a> {
        publish: bool,
        server: &'a str,
        subject: &'a str,
        payload: &'a str,
    }
    
    fn main() {
        let publish = PublishTemplate {
            publish: true,
            server: "localhost",
            subject: "subject_test",
            payload: "test_payload",
        };
    
        let render = publish.render().unwrap();
    
        // write to file
        let mut out_file = File::create("pub.rs").expect("Failed to create file");
        out_file
            .write_all(render.as_bytes())
            .expect("failed to write into file");
    }


<a id="orgbedae95"></a>

### output

pub.rs

    use bytes::Bytes;
    use futures::StreamExt;
    
    #[tokio::main]
    async fn main() -> Result<(), async_nats::Error> {
        let client = async_nats::connect("localhost").await?;
    
        let mut subscriber = client.subscribe("subject_test".into()).await?.take(10);
    
        for _ in 0..10 {
            client.publish("subject_test".into(), "test_payload".into()).await?;
        }
    
        Ok(())
    }


<a id="org67e7abe"></a>

# final notes:

-   all seem to be feature equivalent.
-   tera and askama are fully featured and support inheritance, **filters**, loops &#x2026;
    -   gtmpl documentation and source code (<https://github.com/fiji-flo/gtmpl-rust>) dont mention any of these at all
-   all are similar in performance and minimal performance gains shouldn&rsquo;t matter during the generation process

