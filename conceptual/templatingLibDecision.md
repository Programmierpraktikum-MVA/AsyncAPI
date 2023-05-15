
# Table of Contents

1.  [tera](#org6988f36)
    1.  [general](#org37a91fb)
        1.  [downloads:](#org53374d6)
    2.  [pros](#org944b107)
    3.  [cons](#org590aabe)
    4.  [Examples:](#orgb23047c)
2.  [gtmpl](#orgf655190)
    1.  [general](#orgfba370e)
        1.  [downloads](#org7fcc643)
    2.  [pros](#org5b2c910)
    3.  [cons](#orgefcd08d)
    4.  [Examples:](#org3981908)
3.  [askama](#org20fb09a)
    1.  [general](#org1fc8fe9)
        1.  [downloads](#org21e5538)
    2.  [pros](#orgdfa7a5f)
    3.  [cons](#org01be15d)
    4.  [Examples](#org775eb4f)
        1.  [template example](#org3f9a774)
        2.  [code example](#org524b3ba)
        3.  [output](#org1802621)
4.  [final notes:](#org4e2ec31)


<a id="org6988f36"></a>

# tera


<a id="org37a91fb"></a>

## general

-   jinja2 temlating engine (more common then gotemplate)
-   MIT license
-   2.7k github stars
-   Updated: about 2 months ago


<a id="org53374d6"></a>

### downloads:

-   All-Time: 5,664,947
-   Recent: 773,985


<a id="org944b107"></a>

## pros

-   widely used
-   templating language used alot in python
-   more active community
-   tera-cli
-   simple to use
-   good documentation <https://tera.netlify.app/docs>


<a id="org590aabe"></a>

## cons

-   no type checking


<a id="orgb23047c"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/f6034d5ce5369586e90157f8c3d0e03e5098e8fc/templateTest/src> (doesn&rsquo;t use lazy-statics <https://crates.io/crates/lazy_static> for statics at runtime)


<a id="orgf655190"></a>

# gtmpl


<a id="orgfba370e"></a>

## general

-   gotemplate engine
-   MIT license
-   66 github stars
-   Updated: almost 2 years ago


<a id="org7fcc643"></a>

### downloads

-   All-Time: 130,836
-   Recent: 16,121


<a id="org5b2c910"></a>

## pros

-   go-templates are specified in the doc (may be just a recommendation)
-   better interoperability with go projects
-   used by other project
-   jan gottschick recommended


<a id="orgefcd08d"></a>

## cons

-   seems to change api/struct funcs more often -> upgrading may break has broken things
-   still in 0.7 release and last commit 2 years ago (seems dead?)
-   not used alot
-   no type checking
-   very little documentation <https://docs.rs/crate/gtmpl/latest>
-   documentation doesn&rsquo;t mention filters or inheritance
-   barely used (see github stars)


<a id="org3981908"></a>

## Examples:

<https://github.com/crustacgen/playground/tree/Niclas>


<a id="org20fb09a"></a>

# askama


<a id="org1fc8fe9"></a>

## general

-   jinja2
-   Apache2, MIT dual license
-   2.2k github stars
-   Updated: 2 months ago


<a id="org21e5538"></a>

### downloads

-   All-Time: 3,072,505
-   Recent: 549,705


<a id="orgdfa7a5f"></a>

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
-   no more need for rust<sub>embed</sub>, since everything apart from generation happens at compile time all files are allreay embedded
-   specifically (but not solely) meant for generating rust code from templates
-   very good documentation <https://djc.github.io/askama/>
-   slightly faster (tho that shouldnt really matter)


<a id="org01be15d"></a>

## cons

-   not as matured as tera


<a id="org775eb4f"></a>

## Examples


<a id="org3f9a774"></a>

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


<a id="org524b3ba"></a>

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


<a id="org1802621"></a>

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


<a id="org4e2ec31"></a>

# final notes:

-   tera and askama are fully featured and support inheritance, **filters**, loops &#x2026;
    -   gtmpl documentation and source code (<https://github.com/fiji-flo/gtmpl-rust>) dont mention any of these at all
-   all are similar in performance and minimal performance gains shouldn&rsquo;t matter during the generation process

