# html/semantics/forms/the-input-element/search_input.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/search_input.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>

  <head>
    <title>Search Input</title>
     <meta name=viewport content="width=device-width, maximum-scale=1.0, user-scalable=no" />
     <link rel="author" title="Fabrice Clari" href="mailto:f.clari@inno-group.com">
     <link rel="author" title="Dimitri Bocquet" href="mailto:Dimitri.Bocquet@mosquito-fp7.eu">
     <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-input-element">
     <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-type">
     <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-input-placeholder">
   <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <body>


      <h1>Search Input</h1>
      <input type="search" style="display:none" placeholder="Search..." />

    <div id="log">
    </div>

  <script type="text/javascript">


    test(function() {assert_equals(document.getElementsByTagName("input")[0].type, "search")}, "search type support on input element");
    test(function() {assert_equals(document.getElementsByTagName("input")[0].placeholder, "Search...")}, "placeholder attribute support on input element");

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
        "byte_end": 158,
        "byte_start": 70,
        "col": 6,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 906,
        "byte_start": 875,
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
  "source_name": "html/semantics/forms/the-input-element/search_input.html"
}
```
