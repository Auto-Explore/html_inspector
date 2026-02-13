# html/dom/directionality/bdi-element-invalid-dir.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/directionality/bdi-element-invalid-dir.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta name="author" title="Ryosuke Niwa" href="mailto:rniwa@webkit.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/dom.html#the-directionality">
<link rel="match" href="bdi-element-invalid-dir-ref.html">
<style>
div { position: relative; width: 100px; height: 100px; background: red; }
bdi { width: 100px; height: 100px; display: block; }
span { display: inline-block; width: 50px; height: 100px; background: green; color: green; }
#left { background: green; position: absolute; left: 0px; top: 0px; }
</style>
</head>
<body>
<div><bdi dir="foo"><span id="left"></span><span>&#x05EA;</span></bdi></div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 102,
        "byte_start": 30,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 102,
        "byte_start": 30,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/directionality/bdi-element-invalid-dir.html"
}
```
