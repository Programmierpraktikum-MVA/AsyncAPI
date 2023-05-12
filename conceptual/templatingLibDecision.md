
# Table of Contents

1.  [tera](#org6e04e29)
    1.  [general](#org0e9028b)
        1.  [downloads:](#org9cc41da)
    2.  [pros](#orgd05b81c)
    3.  [cons](#orgf0724fd)
    4.  [Examples:](#org3afaff0)
2.  [gtmpl](#orgf4ba68f)
    1.  [general](#org2a8c542)
        1.  [downloads](#orgfc7ae21)
    2.  [pros](#org70f311a)
    3.  [cons](#orgbc1694a)
    4.  [Examples:](#org015269a)
3.  [askama](#orgaf591f3)
    1.  [general](#org02eb80d)
        1.  [downloads](#orgba45e72)
    2.  [pros](#org20dae80)
    3.  [cons](#org93ff48b)
    4.  [Examples](#orgdaf313b)
        1.  [template example](#org86a5773)
        2.  [code example](#orgfd86233)
        3.  [output](#org10b232e)
4.  [final notes:](#orgbc7144d)


<a id="org6e04e29"></a>

# tera


<a id="org0e9028b"></a>

## general

-   jinja2 temlating engine (more common then gotemplate)
-   MIT license
-   2.7k github stars
-   Updated: about 2 months ago


<a id="org9cc41da"></a>

### downloads:

-   All-Time: 5,664,947
-   Recent: 773,985


<a id="orgd05b81c"></a>

## pros

-   widely used
-   templating language used alot in python
-   more active community
-   tera-cli
-   simple to use
-   good documentation <https://tera.netlify.app/docs>


<a id="orgf0724fd"></a>

## cons

-   some type checking but happens in runtime


<a id="org3afaff0"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/f6034d5ce5369586e90157f8c3d0e03e5098e8fc/templateTest/src> (doesn&rsquo;t use lazy-statics <https://crates.io/crates/lazy_static> for statics at runtime)


<a id="orgf4ba68f"></a>

# gtmpl


<a id="org2a8c542"></a>

## general

-   gotemplate engine
-   MIT license
-   66 github stars
-   Updated: almost 2 years ago


<a id="orgfc7ae21"></a>

### downloads

-   All-Time: 130,836
-   Recent: 16,121


<a id="org70f311a"></a>

## pros

-   go-templates are specified in the doc (may be just a recommendation)
-   better interoperability with go projects
-   used by other project
-   jan gottschick recommended


<a id="orgbc1694a"></a>

## cons

-   seems to change api/struct funcs more often -> upgrading may break has broken things
-   still in 0.7 release and last commit 2 years ago (seems dead?)
-   not used alot
-   less documentation
-   some type checking but happens in runtime
-   very little documentation <https://docs.rs/crate/gtmpl/latest>


<a id="org015269a"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/Niclas>


<a id="orgaf591f3"></a>

# askama


<a id="org02eb80d"></a>

## general

-   jinja2
-   Apache2, MIT dual license
-   2.2k github stars
-   Updated: 2 months ago


<a id="orgba45e72"></a>

### downloads

-   All-Time: 3,072,505
-   Recent: 549,705


<a id="org20dae80"></a>

## pros

-   uses macros under the hood, **&ldquo;generated&rdquo; code type safety assured while compiling generator!!!**
-   specifically meant for generating rust code from templates
-   very good documentation <https://djc.github.io/askama/>
-   slightly faster (tho that shouldnt really matter)


<a id="org93ff48b"></a>

## cons

-   not as matured as tera


<a id="orgdaf313b"></a>

## Examples


<a id="org86a5773"></a>

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


<a id="orgfd86233"></a>

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


<a id="org10b232e"></a>

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


<a id="orgbc7144d"></a>

# final notes:

-   all seem to be feature equivalent.
-   they support inheritance, filters, loops &#x2026;
-   all are similar in performance and minimal performance gains shouldn&rsquo;t matter during the generation process

