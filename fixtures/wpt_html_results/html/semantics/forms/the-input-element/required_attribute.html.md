# html/semantics/forms/the-input-element/required_attribute.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/required_attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>

  <head>
    <title>Required Attribute</title>
    <meta name=viewport content="width=device-width, maximum-scale=1.0, user-scalable=no" />
    <link rel="author" title="Fabrice Clari" href="mailto:f.clari@inno-group.com">
    <link rel="author" title="Dimitri Bocquet" href="mailto:Dimitri.Bocquet@mosquito-fp7.eu">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-input-required">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <body>


      <h1>Required Attribute</h1>
      <div style="display: none">
      <input type="text" required="required" />
    </div>

    <div id="log">
    </div>

  <script type="text/javascript">


    test(function() {assert_equals(document.getElementsByTagName("input")[0].getAttribute("required"), "required")}, "required attribute support on input element");

  </script>

  </body>

</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.viewport.user_scalable_no",
      "message": "Consider avoiding viewport values that prevent users from resizing documents.",
      "severity": "Warning",
      "span": {
        "byte_end": 163,
        "byte_start": 75,
        "col": 5,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 758,
        "byte_start": 727,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/required_attribute.html"
}
```
