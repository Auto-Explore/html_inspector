# html/rendering/sections/headingoffset-and-headingreset-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/sections/headingoffset-and-headingreset-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<link rel="help" href="https://github.com/whatwg/html/pull/11086" />
<title>Test :heading default styles</title>
<div>
  <h1>h1</h1>
  <h2>h2</h2>
  <h3>h3</h3>
  <h4>h4</h4>
  <h5>h5</h5>
  <h6>h6</h6>
  <h6>h7</h6>
  <h6>h8</h6>
  <h6>h9</h6>
</div>
<hr>
<div>
  <h3>h3</h3>
  <h4>h4</h4>
  <h5>h5</h5>
  <h6>h6</h6>
  <h6>h7</h6>
  <h6>h8</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
</div>
<hr>
<div>
  <h5>h5</h5>
  <h6>h6</h6>
  <h6>h7</h6>
  <h6>h8</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
</div>
<hr>
<div>
  <h6>h7</h6>
  <h6>h8</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
</div>
<hr>
<div>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
  <h6>h9</h6>
</div>
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
        "byte_end": 95,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 95,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/rendering/sections/headingoffset-and-headingreset-ref.html"
}
```
