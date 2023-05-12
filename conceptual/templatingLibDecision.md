
# Table of Contents

1.  [tera](#org76bb632)
    1.  [general](#org7236af6)
        1.  [downloads:](#org5c2367e)
    2.  [pros](#org9b137d2)
    3.  [cons](#orgf37feb2)
    4.  [Examples:](#org14adc0d)
2.  [gtmpl](#orgb7b031b)
    1.  [general](#org789513c)
        1.  [downloads](#orgf4d292a)
    2.  [pros](#org8b5b9dc)
    3.  [cons](#orga94a110)
    4.  [Examples:](#orgd4de1e3)
3.  [askama](#org590d548)
    1.  [general](#org1198815)
        1.  [downloads](#orgcad2411)
    2.  [pros](#org5c02850)
    3.  [cons](#org186b217)
    4.  [Examples](#orga9b9b83)
        1.  [template example](#org9c18757)
        2.  [code example](#orgab1f867)
        3.  [output](#org80bac22)
4.  [final notes:](#org4aea262)


<a id="org76bb632"></a>

# tera


<a id="org7236af6"></a>

## general

-   jinja2 temlating engine (more common then gotemplate)
-   MIT license
-   2.7k github stars
-   Updated: about 2 months ago


<a id="org5c2367e"></a>

### downloads:

-   All-Time: 5,664,947
-   Recent: 773,985


<a id="org9b137d2"></a>

## pros

-   widely used
-   templating language used alot in python
-   more active community
-   tera-cli
-   simple to use
-   good documentation <https://tera.netlify.app/docs>


<a id="orgf37feb2"></a>

## cons

-   some type checking but happens in runtime


<a id="org14adc0d"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/f6034d5ce5369586e90157f8c3d0e03e5098e8fc/templateTest/src> (doesn&rsquo;t use lazy-statics <https://crates.io/crates/lazy_static> for statics at runtime)


<a id="orgb7b031b"></a>

# gtmpl


<a id="org789513c"></a>

## general

-   gotemplate engine
-   MIT license
-   66 github stars
-   Updated: almost 2 years ago


<a id="orgf4d292a"></a>

### downloads

-   All-Time: 130,836
-   Recent: 16,121


<a id="org8b5b9dc"></a>

## pros

-   go-templates are specified in the doc (may be just a recommendation)
-   better interoperability with go projects
-   used by other project
-   jan gottschick recommended


<a id="orga94a110"></a>

## cons

-   seems to change api/struct funcs more often -> upgrading may break has broken things
-   still in 0.7 release and last commit 2 years ago (seems dead?)
-   not used alot
-   less documentation
-   some type checking but happens in runtime
-   very little documentation <https://docs.rs/crate/gtmpl/latest>


<a id="orgd4de1e3"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/Niclas>


<a id="org590d548"></a>

# askama


<a id="org1198815"></a>

## general

-   jinja2
-   Apache2, MIT dual license
-   2.2k github stars
-   Updated: 2 months ago


<a id="orgcad2411"></a>

### downloads

-   All-Time: 3,072,505
-   Recent: 549,705


<a id="org5c02850"></a>

## pros

-   uses macros under the hood, **&ldquo;generated&rdquo; code type safety assured while compiling generator!!!**
-   specifically meant for generating rust code from templates
-   very good documentation <https://djc.github.io/askama/>
-   slightly faster (tho that shouldnt really matter)


<a id="org186b217"></a>

## cons

-   not as matured as tera


<a id="orga9b9b83"></a>

## Examples


<a id="org9c18757"></a>

### template example

templates/pub.rs.jinja

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


<a id="orgab1f867"></a>

### code example

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


<a id="org80bac22"></a>

### output

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


<a id="org4aea262"></a>

# final notes:

-   all seem to be feature equivalent.
-   they support inheritance, filters, loops &#x2026;
-   all are similar in performance and minimal performance gains shouldn&rsquo;t matter during the generation process

