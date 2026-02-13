# html/dom/elements/global-attributes/custom-attrs.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/custom-attrs.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>Element Custom Attributes</title>
        <link rel="author" title="Bruno de Oliveira Abinader" href="mailto:bruno.d@partner.samsung.com">
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-dataset">
        <link rel="help" href="https://html.spec.whatwg.org/multipage/#xml">
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
        <script src="/dom/nodes/attributes.js"></script>
    </head>
    <body>
        <h1>Element Custom Attributes</h1>
        <div id="log"></div>
        <script>
            test(function() {
                var div = document.createElement("div");
                div.setAttributeNS("foo", "data-my-custom-attr", "first");
                div.setAttributeNS("bar", "data-my-custom-attr", "second");
                div.dataset.myCustomAttr = "third";

                assert_equals(div.attributes.length, 3);
                attributes_are(div, [["data-my-custom-attr", "first", "foo"],
                                     ["data-my-custom-attr", "second", "bar"],
                                     ["data-my-custom-attr", "third", null]]);
            }, "Setting an Element's dataset property should not interfere with namespaced attributes with same name");
        </script>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/elements/global-attributes/custom-attrs.html"
}
```
