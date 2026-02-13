# html/infrastructure/conformance-requirements/extensibility/foreign.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/conformance-requirements/extensibility/foreign.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en" foo='bar'>
  <head foo='bar'>
    <meta charset="utf-8" foo='bar'>
    <title id='title' foo='bar'>Foreign content</title>
    <link rel="author" title="Philippe Le Hegaret" href="mailto:plh@w3.org"  foo='bar'>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#extensibility"  foo='bar'>
    <script src="/resources/testharness.js"  foo='bar'></script>
    <script src="/resources/testharnessreport.js" foo='bar'></script>
  </head>

  <body foo='bar'>

    <p class='assert' foo='bar'>User agents must treat elements and attributes that they do not understand as semantically neutral; leaving them in the DOM (for DOM processors), and styling them according to CSS (for CSS processors), but not inferring any meaning from them.</p>

    <foo foo='bar' echo>Foobar</foo>

    <div id="log">Running test...</div>

    <script>
      var t = async_test("foreign content");

      on_event(window, "load",
               t.step_func(function() {
                 var nodes = document.getElementsByTagName("*");
                 var cont = true;
                 var last = null;
                 for(var i=0;i<nodes.length && cont; i++) {
                   var as = nodes.item(i).getAttribute("foo");
                   if (!(as === "bar") && (nodes.item(i).getAttribute("id") === "log")) {
                     cont = false;
                   } else {
                     last = nodes.item(i);
                     assert_equals(as, "bar");
                   }
                 }

                 assert_equals(last.nodeName, "FOO");
                 assert_equals(last.getAttribute("echo"), "");
                 assert_equals(last.getAttribute("charly"), null);
                 t.done();
               }));
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “foo” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 807,
        "byte_start": 787,
        "col": 5,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “foo” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 807,
        "byte_start": 787,
        "col": 5,
        "line": 16
      }
    }
  ],
  "source_name": "html/infrastructure/conformance-requirements/extensibility/foreign.html"
}
```
